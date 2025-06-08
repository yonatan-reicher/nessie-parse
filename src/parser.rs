use crate::position::Pos;
use crate::state::State;

use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseResult<T, E, F = ()> {
    /// Called on a successful parse!
    Ok(T, Pos),
    /// The parser could not parse the input, perhaps another parser could.
    /// (This is used to implement backtracking)
    Fail(F, Pos),
    /// The parser expected it could succeed, but it did not.
    Err(E, Pos),
}

/// A parser is a function from a `State` to a `ParseResult`.
/// The `T` type is the type of the value produced by the parser, the `E` is the
/// type of the errors it can produce. `F` is the type of failure - this is like
/// an error, but made to be used for backtracking. It is optional.
/// The `'a` lifetime is the lifetime of the parser.
#[derive(Clone)]
pub struct Parser<'a, T, E, F = ()> {
    /// This name is useful for debugging.
    name: Rc<String>,
    // Maybe here we might want to use a different lifetime?  --v
    parse: Rc<dyn Fn(State<'a>) -> ParseResult<T, E, F> + 'a>,
}

impl<T, E, F> std::fmt::Debug for Parser<'_, T, E, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.as_ref();
        write!(f, "Parser(\"{name}\")")
    }
}

impl<'a, T, E, F> Parser<'a, T, E, F> {
    // Here `T` is bound by `'a` because we are storing a `T` in the returned
    // parser.
    pub fn ret(value: T) -> Self
    where
        T: Clone + 'a,
    {
        Parser {
            name: Rc::new("ret".to_string()),
            parse: Rc::new(move |state| ParseResult::Ok(value.clone(), state.pos)),
        }
    }

    pub fn fail(value: F) -> Self
    where
        F: Clone + 'a,
    {
        Parser {
            name: Rc::new("fail".to_string()),
            parse: Rc::new(move |state| ParseResult::Fail(value.clone(), state.pos)),
        }
    }

    pub fn fail_with(value: impl Fn() -> F + 'a) -> Self {
        Parser {
            name: Rc::new("fail".to_string()),
            parse: Rc::new(move |state| ParseResult::Fail(value(), state.pos)),
        }
    }

    pub fn err(value: E) -> Self
    where
        E: Clone + 'a,
    {
        Parser {
            name: Rc::new("err".to_string()),
            parse: Rc::new(move |state| ParseResult::Err(value.clone(), state.pos)),
        }
    }

    pub fn from_fn<Func>(func: Func) -> Self
    where
        Func: Fn(State<'a>) -> ParseResult<T, E, F> + 'a,
    {
        Parser {
            name: Rc::new("from_fn".to_string()),
            parse: Rc::new(func),
        }
    }

    pub fn parse(&self, state: State<'a>) -> ParseResult<T, E, F> {
        (self.parse)(state)
    }

    // More advanced constructors and combinators.

    /// This is useful for debugging parsers.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Rc::new(name.into());
        self
    }

    pub fn map<U>(self, f: impl Fn(T) -> U + 'a) -> Parser<'a, U, E, F>
    where
        F: 'a,
        E: 'a,
        T: 'a,
    {
        let name = format!("map({})", self.name);
        Parser::from_fn(move |state| match self.parse(state) {
            ParseResult::Ok(value, pos) => ParseResult::Ok(f(value), pos),
            ParseResult::Fail(fail_value, pos) => ParseResult::Fail(fail_value, pos),
            ParseResult::Err(err_value, pos) => ParseResult::Err(err_value, pos),
        })
        .with_name(name)
    }

    pub fn and_then<U, Func>(self, func: Func) -> Parser<'a, U, E, F>
    where
        Func: Fn(T) -> Parser<'a, U, E, F> + 'a,
        F: 'a,
        E: 'a,
        T: 'a,
    {
        let name = format!("and_then({})", self.name);
        Parser::from_fn(move |state| match self.parse(state) {
            ParseResult::Ok(value, pos) => func(value).parse(state.with_pos(pos)),
            ParseResult::Fail(fail_value, pos) => ParseResult::Fail(fail_value, pos),
            ParseResult::Err(err_value, pos) => ParseResult::Err(err_value, pos),
        })
        .with_name(name)
    }

    pub fn or<G, H>(
        self,
        other: Parser<'a, T, E, G>,
        combine_fails: impl Fn(F, State<'a>, G, State<'a>) -> H + 'a,
    ) -> Parser<'a, T, E, H>
    where
        T: 'a,
        E: 'a,
        F: 'a,
        G: 'a,
    {
        let name = format!("or({} | {})", self.name, other.name);
        Parser::from_fn(move |state| match self.parse(state) {
            ParseResult::Ok(value, pos) => ParseResult::Ok(value, pos),
            ParseResult::Fail(f1, f1_pos) => {
                match other.parse(state) {
                    ParseResult::Ok(value, pos) => ParseResult::Ok(value, pos),
                    ParseResult::Fail(f2, f2_pos) => {
                        let f =
                            combine_fails(f1, state.with_pos(f1_pos), f2, state.with_pos(f2_pos));
                        // The position of the failure will just be at the start
                        ParseResult::Fail(f, state.pos)
                    }
                    ParseResult::Err(err_value, pos) => ParseResult::Err(err_value, pos),
                }
            }
            ParseResult::Err(err_value, pos) => ParseResult::Err(err_value, pos),
        })
        .with_name(name)
    }

    pub fn one_of(
        parsers: impl IntoIterator<Item = Parser<'a, T, E, F>> + 'a,
    ) -> Parser<'a, T, E, Vec<(F, Pos)>>
    where
        T: 'a,
        E: 'a,
        F: 'a,
    {
        let mut ret = Parser::fail_with(|| vec![]);
        let mut names = vec![];
        for parser in parsers {
            names.push(parser.name.clone());
            ret = ret.or(parser, |f1, _f1_state, f2, f2_state| {
                let mut f = f1;
                f.push((f2, f2_state.pos));
                f
            });
        }
        let name = format!(
            "OneOf({})",
            names
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
        ret.with_name(name)
    }

    pub fn filter(self, pred: impl Fn(&T) -> bool + 'a, fail: F) -> Self
    where
        T: Clone + 'a,
        E: Clone + 'a,
        F: Clone + 'a,
    {
        let name = format!("filter({})", self.name);
        Parser::and_then(self, move |value| {
            if pred(&value) {
                Parser::ret(value)
            } else {
                Parser::fail(fail.clone())
            }
        })
        .with_name(name)
    }
}

impl<'a, E, F> Parser<'a, State<'a>, E, F> {
    /// A parser that just returns the current state.
    pub fn state() -> Self {
        Parser {
            name: Rc::new("state".to_string()),
            parse: Rc::new(|state| ParseResult::Ok(state, state.pos)),
        }
    }
}


#[macro_export]
macro_rules! one_of {
    [ $($parser:expr),* $(,)? ] => {{
        $crate::parser::Parser::one_of(vec![$($parser),*])
    }};
}
