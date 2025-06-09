use crate::parser::Parser;
use crate::state::State;

impl<'a, T: 'a, F: 'a, E: 'a> Parser<'a, T, E, F> {
    pub fn of_bool(value: bool) -> Parser<'a, T, E, F>
    where
        T: Default + Clone,
        F: Default + Clone,
    {
        if value {
            Parser::ret(T::default())
        } else {
            Parser::fail(F::default())
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EofFailure;

impl<'a, E: 'a> Parser<'a, char, E, EofFailure> {
    pub fn char() -> Parser<'a, char, E, EofFailure> {
        Parser::from_fn(|state| {
            if state.rest().is_empty() {
                ParseResult::Fail(EofFailure, state.pos)
            } else {
                let ch = state.rest().chars().next().unwrap();
                ParseResult::Ok(ch, state.pos.right())
            }
        })
    }
}

impl<'a, F: Clone + Default + 'a, E: 'a> Parser<'a, char, E, F> {
    pub fn char_eq(ch: char) -> Parser<'a, char, E, F> {
        Parser::char()
            .map_fail(|_| F::default())
            .filter(move |&c| c == ch, F::default())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotWhitespace;

impl<'a, E: 'a> Parser<'a, char, E, NotWhitespace> {
    pub fn whitespace() -> Parser<'a, char, E, NotWhitespace> {
        Parser::char()
            .map_fail(|_| NotWhitespace)
            .filter(|c| c.is_whitespace(), NotWhitespace)
    }
}

impl<'a, F: 'a, E: 'a> Parser<'a, (), E, F> {
    pub fn skip_whitespace() -> Parser<'a, (), E, F> {
        Parser::whitespace().repeat_0().map(|_| ())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotALetter;

impl<'a, E: 'a> Parser<'a, char, E, NotALetter> {
    pub fn letter() -> Parser<'a, char, E, NotALetter> {
        Parser::char()
            .map_fail(|_| NotALetter)
            .filter(|c| c.is_ascii_alphabetic(), NotALetter)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotADigit;

impl<'a, E: 'a> Parser<'a, char, E, NotADigit> {
    pub fn digit() -> Parser<'a, char, E, NotADigit> {
        Parser::char()
            .map_fail(|_| NotADigit)
            .filter(|c| c.is_ascii_digit(), NotADigit)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotFound;

impl<'a, E: 'a> Parser<'a, char, E, NotFound> {
    pub fn eof() -> Parser<'a, (), E, NotFound> {
        Parser::state()
            .map(|state| state.rest().is_empty())
            .and_then(Parser::of_bool)
    }

    pub fn expect_string(expected: &'static str) -> Parser<'a, (), E, NotFound> {
        Parser::state()
            .map(move |state: State| state.rest().starts_with(expected))
            .and_then(Parser::of_bool)
    }
}

impl<'a, T, F, E> Parser<'a, T, E, F> {
    /// Repeats this parser zero or one times.
    pub fn maybe<G>(self) -> Parser<'a, Option<T>, E, G>
    where
        T: 'a,
        E: 'a,
        F: 'a,
    {
        let name = format!("maybe({})", &self.name);
        one_of![self.map(Some), Parser::ret_with(|| None)]
            .map_fail(|_| panic!("maybe should not fail"))
            .with_name(name)
    }

    /// Repeats this parser zero or more times.
    pub fn repeat_0<G>(self) -> Parser<'a, Vec<T>, E, G>
    where
        T: Clone + 'a,
        E: 'a,
        F: 'a,
    {
        let name = format!("repeat_0({})", &self.name);
        one_of![
            self.clone().and_then(move |x| {
                self.clone()
                    .repeat_0()
                    .map(move |xs| vec![x.clone()].mutate(|v| v.extend(xs)))
            }),
            Parser::ret_with(Vec::new)
        ]
        .map_fail(|_| panic!("repeat_0 should not fail"))
        .with_name(name)
    }

    /// Repeats this parser one or more times.
    pub fn repeat_1(self) -> Parser<'a, Vec<T>, E, F>
    where
        T: Clone + 'a,
        E: 'a,
        F: 'a,
    {
        let name = format!("repeat_1({})", &self.name);
        self.clone()
            .and_then(move |x| {
                self.clone()
                    .repeat_0()
                    .map(move |xs| vec![x.clone()].mutate(|v| v.extend(xs)))
            })
            .with_name(name)
    }
}

impl<'a, T, F, E> Parser<'a, T, E, F> {
    #[allow(clippy::should_implement_trait)]
    pub fn not<G>(self) -> Parser<'a, (), E, G>
    where
        T: 'a,
        E: 'a,
        F: 'a,
        G: Default + 'a,
    {
        #[rustfmt::skip]
        #[derive(Debug, Clone)]
        enum ShouldFail { Yes, No }

        let name = format!("not({})", &self.name);
        self.map_fail(|_| ShouldFail::No)
            .and_then(|_| Parser::fail(ShouldFail::Yes))
            .and_then_fail(|f| match f {
                ShouldFail::Yes => Parser::fail_with(|| G::default()),
                ShouldFail::No => Parser::ret(()),
            })
            .with_name(name)
    }
}
