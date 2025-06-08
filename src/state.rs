use std::borrow::Borrow;

use rpds::List;

#[derive(Debug, Clone)]
pub struct State<'a, Token, Warning> {
    tokens: &'a [Token],
    warnings: List<Warning>,
}

impl<'a, Token, Warning> State<'a, Token, Warning> {
    pub fn pop(&mut self) -> Option<&Token> {
        self.tokens.first().map(|t| {
            self.tokens = &self.tokens[1..];
            t
        })
    }
}

impl<'a, Token, Warning> State<'a, Token, Warning> {
    pub fn pop_eq<D: PartialEq + ?Sized>(&mut self, token: &D) -> bool
    where
        Token: Borrow<D>,
    {
        self.tokens.first().is_some_and(|t| t.borrow() == token)
    }
}
