use crate::position::Pos;

#[derive(Debug, Clone, Copy)]
pub struct State<'input> {
    pub text: &'input str,
    pub pos: Pos,
}

impl<'input> State<'input> {
    pub const fn new(text: &'input str) -> Self {
        let pos = Pos::start();
        State { text, pos }
    }

    pub const fn at(text: &'input str, pos: Pos) -> Self {
        State { text, pos }
    }

    pub const fn with_pos(self, pos: Pos) -> Self {
        State { pos, ..self }
    }

    pub fn rest(&self) -> &'input str {
        &self.text[self.pos.offset..]
    }

    pub const fn eof(&self) -> bool {
        self.pos.offset >= self.text.len()
    }
}

impl<'input, T: Into<&'input str>> From<T> for State<'input> {
    fn from(text: T) -> Self {
        State::new(text.into())
    }
}
