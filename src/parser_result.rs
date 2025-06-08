use crate::state::State;

pub enum ParserResult<'a, T, Token, Warning, Error> {
    Parsed(T, State<'a, Token, Warning>),
    Err(Error),
}

