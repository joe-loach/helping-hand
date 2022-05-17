use crate::{Atom, Stmt, AtomKind};

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

    pub fn nth(&self, n: usize) -> Option<AtomKind> {
        self.stmt.get(self.pos + n).map(|at| at.kind)
    }

    pub fn current(&self) -> Option<AtomKind> {
        self.nth(0)
    }

    pub fn at(&self, kind: AtomKind) -> bool {
        self.current().map(|at| at == kind).unwrap_or(false)
    }

    pub fn data(&self) -> Option<u32> {
        self.stmt.get(self.pos).map(|at| at.data)
    }

    pub fn eat(&mut self, kind: AtomKind) -> Option<u32> {
        if !self.at(kind) {
            return None;
        }
        let data = self.data().unwrap();
        self.pos += 1;
        Some(data)
    }

    pub fn eat_any(&mut self) -> Option<u32> {
        self.current().and_then(|kind| self.eat(kind))
    }

    pub fn eat_while(&mut self, pred: impl Fn(AtomKind) -> bool) -> Vec<u32> {
        let mut data = vec![];
        while let Some(kind) = self.current() {
            if pred(kind) {
                data.push(self.bump(kind));
            } else {
                break;
            }
        }
        data
    }

    pub fn bump(&mut self, kind: AtomKind) -> u32 {
        match self.eat(kind) {
            Some(data) => data,
            None => panic!("bumping {kind:?} @ {} failed", self.pos),
        }
    }

    pub fn finished(&self) -> bool {
        self.current().is_none()
    }
}

impl<'a> Iterator for Cursor<'a> {
    type Item = Atom;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(atom) = self.stmt.get(self.pos) {
            self.pos += 1;
            Some(atom)
        } else {
            None
        }
    }
}
