use crate::{Atom, Stmt};

pub struct Checkpoint(usize);

impl Checkpoint {
    pub fn start() -> Self {
        Checkpoint(0)
    }
}

pub struct Cursor<'a> {
    pos: usize,
    stmt: &'a Stmt<'a>,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(stmt: &'a Stmt<'a>) -> Self {
        Cursor { pos: 0, stmt }
    }

    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint(self.pos)
    }

    pub fn rewind(&mut self, point: Checkpoint) {
        self.pos = point.0;
    }

    pub fn nth(&self, n: usize) -> Option<Atom> {
        self.stmt.get(self.pos + n).map(|(atom, _)| atom)
    }

    pub fn current(&self) -> Option<Atom> {
        self.nth(0)
    }

    pub fn data(&self) -> Option<u32> {
        self.stmt.get(self.pos).map(|(_, data)| data)
    }

    pub fn at(&self, atom: Atom) -> bool {
        self.current().map(|it| it == atom).unwrap_or(false)
    }

    pub fn eat(&mut self, atom: Atom) -> Option<u32> {
        if !self.at(atom) {
            return None;
        }
        let data = self.data()?;
        self.pos += 1;
        Some(data)
    }

    pub fn eat_any(&mut self) -> Option<u32> {
        self.current().and_then(|curr| self.eat(curr))
    }

    pub fn eat_while(&mut self, pred: impl Fn(Atom) -> bool) -> Vec<u32> {
        let mut data = vec![];
        while let Some(c) = self.current() {
            if pred(c) {
                data.push(self.bump(c));
            } else {
                break;
            }
        }
        data
    }

    pub fn bump(&mut self, atom: Atom) -> u32 {
        match self.eat(atom) {
            Some(n) => n,
            None => panic!("bumping {atom:?} @ {} failed", self.pos),
        }
    }

    pub fn finished(&self) -> bool {
        self.current().is_none()
    }
}

impl<'a> Iterator for Cursor<'a> {
    type Item = (Atom, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pair) = self.stmt.get(self.pos) {
            self.pos += 1;
            Some(pair)
        } else {
            None
        }
    }
}
