use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};

use crate::inner::UnusedInner;

/// A container for unused generic types
///
/// This type is exported as [`Unused`](type@crate::Unused). The `Unused`
/// variant is also exported under the same name.
pub enum UnusedImpl<T: UnusedInner> {
    Unused,
    __Inconstruable(T),
}

impl<T: UnusedInner> fmt::Debug for UnusedImpl<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Unused").finish()
    }
}

impl<T: UnusedInner> Clone for UnusedImpl<T> {
    fn clone(&self) -> Self {
        Self::Unused
    }

    fn clone_from(&mut self, _source: &Self) {}
}

impl<T: UnusedInner> Copy for UnusedImpl<T> {}

impl<T: UnusedInner> Default for UnusedImpl<T> {
    fn default() -> Self {
        Self::Unused
    }
}

impl<T: UnusedInner> PartialEq for UnusedImpl<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T: UnusedInner> Eq for UnusedImpl<T> {}

impl<T: UnusedInner> PartialOrd for UnusedImpl<T> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<T: UnusedInner> Ord for UnusedImpl<T> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<T: UnusedInner> Hash for UnusedImpl<T> {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}
