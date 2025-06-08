mod state;
mod parser_result;
pub mod parser;

pub use state::State;
pub use parser::{Parser, ParserExt};

pub type TokenRange = std::ops::Range<usize>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
