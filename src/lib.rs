//! # Nanostr
//!
//! [`NanoString`] is a 16 byte long constant string type.
//!
//! # When do I want to use it
//!
//! If you have usually very small string values (15 characters or less) and want to use them inside a
//! [`std::collections::HashMap`] or [`std::collections::BTreeMap`] as a key,
//! you should use [`NanoString`]

use core::fmt;
use core::{ops::Deref, usize};

mod inner;
use inner::{Inline, NanoStringInner, Owned};

#[derive(Clone)]
pub struct NanoString {
    inner: NanoStringInner,
}

impl NanoString {
    pub fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }

    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    pub fn len(&self) -> usize {
        self.as_bytes().len()
    }

    pub const fn new(&self) -> NanoString {
        NanoString {
            inner: NanoStringInner::Inline(Inline::new()),
        }
    }
}

impl AsRef<str> for NanoString {
    #[inline]
    fn as_ref(&self) -> &str {
        self
    }
}

impl AsRef<[u8]> for NanoString {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Deref for NanoString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        match &self.inner {
            NanoStringInner::Inline(inline) => inline.as_str(),
            NanoStringInner::Owned(owned) => owned.as_str(),
        }
    }
}

impl From<&str> for NanoString {
    fn from(s: &str) -> Self {
        if let Some(inline) = Inline::try_from_str(s) {
            NanoString {
                inner: NanoStringInner::Inline(inline),
            }
        } else {
            NanoString {
                inner: NanoStringInner::Owned(Owned::from_str(s)),
            }
        }
    }
}

impl Default for NanoString {
    fn default() -> Self {
        NanoString::from("")
    }
}

impl fmt::Display for NanoString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl fmt::Debug for NanoString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::NanoString;

    #[test]
    fn size() {
        assert_eq!(16, std::mem::size_of::<NanoString>());
    }
}
