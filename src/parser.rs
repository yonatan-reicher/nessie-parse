use std::borrow::Borrow;

use crate::state::State;

pub trait Parser<Token, Warning, Error, Output> {
    fn parse(&self, state: &mut State<Token, Warning>) -> Result<Output, Error>;
}

mod from_fn;
pub use from_fn::from_fn;

mod ret_fail;
pub use ret_fail::{fail, ret};

mod cases;
pub use cases::cases;

pub trait ParserExt<T, W, E, O>: Parser<T, W, E, O> + Sized {
    fn map<U>(self, f: impl Fn(O) -> U) -> impl Parser<T, W, E, U> {
        from_fn(move |state| self.parse(state).map(&f))
    }

    fn and_then<P, U>(self, f: impl Fn(O) -> P) -> impl Parser<T, W, E, U>
    where
        P: Parser<T, W, E, U>,
    {
        from_fn(move |state| {
            let o = self.parse(state)?;
            f(o).parse(state)
        })
    }
}

pub trait OptionParserExt<T, W, E, O>: Parser<T, W, E, Option<O>> + Sized {
    fn some_or_err<E1>(self, gen: impl Fn() -> E1) -> impl Parser<T, W, E, O>
    where
        E: From<E1>,
    {
        from_fn(move |state| {
            let a = self.parse(state)?;
            a.ok_or_else(&gen).map_err(E1::into)
        })
    }
}

impl<T, W, E, O, P> ParserExt<T, W, E, O> for P where P: Parser<T, W, E, O> {}
impl<T, W, E, O, P> OptionParserExt<T, W, E, O> for P where P: Parser<T, W, E, Option<O>> {}

pub fn is_eq<'a, Token, Warning: 'a, Error: 'a, D>(
    x: &'a D,
) -> impl Parser<Token, Warning, Error, bool> + 'a
where
    Token: Borrow<D> + 'a,
    D: PartialEq + ?Sized,
{
    from_fn(move |state| Ok(state.pop_eq(x)))
}

pub fn token<Token: Clone, Warning, Error>() -> impl Parser<Token, Warning, Error, Option<Token>> {
    from_fn(|state| Ok(state.pop().cloned()))
}
