use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Inline<const N: usize>([u8; N]);

impl<const N: usize> Inline<N> {
    pub const fn new() -> Inline<N> {
        Inline([0u8; N])
    }

    /// Will return Some(Inline(..)), if the slice will fit into the string.
    pub fn try_from_str(s: &str) -> Option<Inline<N>> {
        if s.len() <= N && !s.as_bytes().contains(&0u8) {
            let mut inline = Inline([0u8; N]);
            inline.0[..s.len()].copy_from_slice(&s.as_bytes()[..s.len()]);
            Some(inline)
        } else {
            None
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        let end = self.0.iter().position(|b| *b == 0u8).unwrap_or(N);
        &self.0[..end]
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.as_bytes())
            .expect("invariant violated: Inline string does not contain valid UTF-8")
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Owned(Arc<Box<str>>);

impl Owned {
    pub fn from_str(s: &str) -> Owned {
        Owned(Arc::new(Box::from(s)))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone)]
pub(crate) enum NanoStringInner {
    Inline(Inline<15>),
    Owned(Owned),
}
