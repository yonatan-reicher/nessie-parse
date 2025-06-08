use crate::{Parser, State};

struct FromFn<F> {
    func: F,
}

impl<T, W, E, O, F> Parser<T, W, E, O> for FromFn<F>
where
    F: Fn(&mut State<T, W>) -> Result<O, E>,
{
    fn parse(&self, state: &mut State<T, W>) -> Result<O, E> {
        (self.func)(state)
    }
}

pub fn from_fn<Token, Warning, Error, Output>(
    func: impl Fn(&mut State<Token, Warning>) -> Result<Output, Error>,
) -> impl Parser<Token, Warning, Error, Output> {
    FromFn { func }
}
