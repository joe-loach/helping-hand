use crate::{cursor::Cursor, Atom};

pub struct Stmt<'a>(&'a [Atom], &'a [u32]);

impl Stmt<'_> {
    pub fn get(&self, index: usize) -> Option<(Atom, u32)> {
        self.0.get(index).map(|&atom| (atom, self.1[index]))
    }

    pub fn atoms(&self) -> &[Atom] {
        self.0
    }

    pub fn data(&self) -> &[u32] {
        self.1
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Atom, &u32)> {
        self.atoms().iter().zip(self.data().iter())
    }

    pub fn cursor(&self) -> Cursor {
        Cursor::new(self)
    }
}

pub struct IR {
    stmts: Vec<u32>,
    atoms: Vec<Atom>,
    data: Vec<u32>,
    errors: Vec<String>,
}

impl IR {
    pub(crate) fn new() -> Self {
        IR {
            stmts: vec![0],
            atoms: Vec::new(),
            data: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub(crate) fn finish(&mut self) {
        self.stmts.push(self.atoms.len() as u32);
    }

    pub(crate) fn push(&mut self, atom: Atom, data: u32) {
        self.atoms.push(atom);
        self.data.push(data);
    }

    pub(crate) fn error(&mut self, msg: impl Into<String>) {
        self.errors.push(msg.into());
        self.push(Atom::Error, self.errors.len() as u32);
    }

    pub fn stmt(&self, i: usize) -> Option<Stmt<'_>> {
        if i + 1 < self.stmts.len() {
            let start = self.stmts[i] as usize;
            let end = self.stmts[i + 1] as usize;
            Some(Stmt(&self.atoms[start..end], &self.data[start..end]))
        } else {
            None
        }
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }
}

impl<'a> IntoIterator for &'a IR {
    type Item = Stmt<'a>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

pub struct Iter<'a> {
    ir: &'a IR,
    pos: usize,
}

impl<'a> Iter<'a> {
    pub fn new(ir: &'a IR) -> Self {
        Self { ir, pos: 0 }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Stmt<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(stmt) = self.ir.stmt(self.pos) {
            self.pos += 1;
            Some(stmt)
        } else {
            None
        }
    }
}
