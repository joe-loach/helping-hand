pub(crate) struct Binary {
    inner: Vec<u8>,
}

impl Binary {
    pub(crate) fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub(crate) fn push(&mut self, data: u32) {
        // this compiler is little-endian
        let bytes = data.to_le_bytes();
        self.inner.extend_from_slice(&bytes);
    }

    pub(crate) fn push_byte(&mut self, byte: u8) {
        self.inner.push(byte);
    }

    pub(crate) fn extend_with(&mut self, bytes: &[u8]) {
        self.inner.extend_from_slice(bytes)
    }

    pub(crate) fn extend_with_n(&mut self, n: usize, byte: u8) {
        self.inner.extend((0..n).map(|_| byte));
    }

    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }

    pub(crate) fn into_vec(self) -> Vec<u8> {
        self.inner
    }
}