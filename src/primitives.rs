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
        let get_char = |state: State| state.rest().chars().next();
        Parser::state().and_then(move |state| match get_char(state) {
            None => Parser::fail(EofFailure),
            Some(ch) => Parser::ret(ch),
        })
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
