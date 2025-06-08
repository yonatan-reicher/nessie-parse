use super::Parser;
use crate::state::State;

struct Ret<F> {
    gen: F,
}

struct Fail<F> {
    gen: F,
}

impl<T, W, E, O, F> Parser<T, W, E, O> for Ret<F>
where
    F: Fn() -> O,
{
    fn parse(&self, _: &mut State<T, W>) -> Result<O, E> {
        Ok((self.gen)())
    }
}

impl<T, W, E, O, F> Parser<T, W, E, O> for Fail<F>
where
    F: Fn() -> E,
{
    fn parse(&self, _: &mut State<T, W>) -> Result<O, E> {
        Err((self.gen)())
    }
}

pub fn ret<T, W, E, O, F: Fn() -> O>(gen: F) -> impl Parser<T, W, E, O> {
    Ret { gen }
}

pub fn fail<T, W, E, O, F: Fn() -> E>(gen: F) -> impl Parser<T, W, E, O> {
    Fail { gen }
}
