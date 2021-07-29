use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};

use crate::inner::UnusedInner;
use crate::End;

/// An `UnusedInner` that is contravariant over `T`
pub struct Contravariant<T: ?Sized, N: UnusedInner> {
    _t: fn(T),
    next: N,
}

impl<T: ?Sized, N: UnusedInner> UnusedInner for Contravariant<T, N> {
    fn inconstruable(self) -> End {
        self.next.inconstruable()
    }
}

impl<T: ?Sized, N: UnusedInner> fmt::Debug for Contravariant<T, N> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inconstruable() {}
    }
}

impl<T: ?Sized, N: UnusedInner> fmt::Display for Contravariant<T, N> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inconstruable() {}
    }
}

impl<T: ?Sized, N: UnusedInner> Clone for Contravariant<T, N> {
    fn clone(&self) -> Self {
        match self.inconstruable() {}
    }
}

impl<T: ?Sized, N: UnusedInner> Copy for Contravariant<T, N> {}

impl<T: ?Sized, N: UnusedInner> PartialEq for Contravariant<T, N> {
    fn eq(&self, _other: &Self) -> bool {
        match self.inconstruable() {}
    }
}

impl<T: ?Sized, N: UnusedInner> Eq for Contravariant<T, N> {}

impl<T: ?Sized, N: UnusedInner> PartialOrd for Contravariant<T, N> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        match self.inconstruable() {}
    }
}

impl<T: ?Sized, N: UnusedInner> Ord for Contravariant<T, N> {
    fn cmp(&self, _other: &Self) -> Ordering {
        match self.inconstruable() {}
    }
}

impl<T: ?Sized, N: UnusedInner> Hash for Contravariant<T, N> {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        match self.inconstruable() {}
    }
}
