use crate::State;

pub trait CombineFail<'a, Rhs, Out> {
    fn combine_fail(self, self_state: State<'a>, other: Rhs, other_state: State<'a>) -> Out;
}

pub trait CombineManyFail<'text, Out>: Sized {
    fn combine_many_fail(fails: Vec<(Self, State<'text>)>) -> Out;
}

// Let's have some useful implementations!

// Implementations for combining to ().

impl<F1, F2> CombineFail<'_, F2, ()> for F1 {
    fn combine_fail(self, _: State, _: F2, _: State) {}
}

impl<F> CombineManyFail<'_, ()> for F {
    fn combine_many_fail(_: Vec<(Self, State)>) {}
}

// Now for just saving everything

impl<'a, F1, F2> CombineFail<'a, F2, (F1, State<'a>, F2, State<'a>)> for F1 {
    fn combine_fail(
        self,
        s1: State<'a>,
        other: F2,
        s2: State<'a>,
    ) -> (F1, State<'a>, F2, State<'a>) {
        (self, s1, other, s2)
    }
}

impl<'a, F> CombineManyFail<'a, Vec<(F, State<'a>)>> for F {
    fn combine_many_fail(fails: Vec<(Self, State)>) -> Vec<(Self, State)> {
        fails
    }
}

// Now for saving the failure, but not the parsing state.

impl<F1, F2> CombineFail<'_, F2, (F1, F2)> for F1 {
    fn combine_fail(self, _: State, other: F2, _: State) -> (F1, F2) {
        (self, other)
    }
}

impl<F> CombineManyFail<'_, Vec<F>> for F {
    fn combine_many_fail(fails: Vec<(Self, State)>) -> Vec<Self> {
        fails.into_iter().map(|(f, _)| f).collect()
    }
}
