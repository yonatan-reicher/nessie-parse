use super::Parser;
use crate::state::State;

struct Ret<F> {
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

pub fn ret<T, W, E, O, F: Fn() -> O>(gen: F) -> impl Parser<T, W, E, O> {
    Ret { gen }
}
