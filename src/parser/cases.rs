use super::from_fn;
use crate::parser::{self, Parser};

struct Case<T, W, E, O>(Box<dyn Parser<T, W, E, Option<O>>>);

struct CasesBuilder<T, W, E, O> {
    cases: Vec<Case<T, W, E, O>>,
}

pub fn cases<T, W, E, O>() -> CasesBuilder<T, W, E, O> {
    CasesBuilder { cases: vec![] }
}

impl<T, W, E, O> CasesBuilder<T, W, E, O> {
    pub fn case<Data, Body>(
        self,
        cond: impl Parser<T, W, E, Option<Data>> + 'static,
        body: impl Fn(Data) -> Body + 'static,
    ) -> Self
    where
        Body: Parser<T, W, E, O>,
        T: 'static,
        W: 'static,
        E: 'static,
        O: 'static,
    {
        let case = Case(Box::new(from_fn(move |state| {
            let data = cond.parse(state)?;
            data.map(|data| body(data).parse(state)).transpose()
        })));
        let mut cases = self.cases;
        cases.push(case);
        CasesBuilder { cases }
    }

    pub fn default(self, body: impl Parser<T, W, E, O>) -> impl Parser<T, W, E, O> {
        let cases = self.cases;
        super::from_fn(move |input| {
            for case in cases.iter() {
                if case.cond.parse(input)? {
                    return case.body.parse(input);
                }
            }
            body.parse(input)
        })
    }
}
