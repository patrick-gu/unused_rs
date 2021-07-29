use core::fmt;

use crate::inner::UnusedInner;

/// An `UnusedInner` that does not contain a type.
///
/// `End` is used to denote the end of a chain of `UnusedInner`s.
///
/// This is similar to [`Infallible`](core::convert::Infallible)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum End {}

impl UnusedInner for End {
    fn inconstruable(self) -> End {
        self
    }
}

impl fmt::Display for End {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}
