//! # About Unused
//!
//! The [`unused`](./index.html) crate provides [`Unused`], a struct that
//! allows types to have unused generic parameters that do not act like they
//! are owned.
//!
//! Unlike [`PhantomData<T>`], [`Unused<T>`] does not tell the compiler that
//! the type owns a `T`.
//!
//! Because [`Unused<T>`] does not "own" a `T`, it is [`Send`], [`Sync`], and
//! [`Unpin`] even if `T` is not. It is just a container for the `T` type, and
//! has data or state of its own. All instances of it are the same.
//!
//! ## Feedback
//! If you experience any issues or have any feedback, please feel free to open
//! an issue on the
//! [GitHub repository](https://github.com/patrick-gu/unused_rs/issues/new).
//!
//! ## `no_std` Support
//!
//! [`unused`](./index.html) optionally supports `no_std`. Disable the default
//! crate features to enable this.
//!
//! ## Example
//!
//! Consider a trait `Producer`.
//!
//! We implement `Producer` for a struct `ProducesFive`, and for `usize`.
//!
//! ```
//! trait Producer {
//!     type Output;
//!
//!     fn produce(&self, data: &str) -> Self::Output;
//! }
//!
//! struct ProducesFive;
//!
//! impl Producer for ProducesFive {
//!     type Output = i32;
//!
//!     fn produce(&self, _data: &str) -> Self::Output {
//!         5
//!     }
//! }
//!
//! impl Producer for usize {
//!     type Output = usize;
//!
//!     fn produce(&self, _data: &str) -> Self::Output {
//!         *self
//!     }
//! }
//! ```
//!
//! Now, let's say that we want to have a `Producer` for any type that
//! implements [`FromStr`](core::str::FromStr) by parsing the provided `data`.
//! We create a `FromStrProducer` struct, to not conflict with the existing
//! implementation for [`usize`].
//!
//! ```
//! use std::str::FromStr;
//!
//! struct FromStrProducer<T: FromStr>;
//!
//! impl<T: FromStr> Producer for FromStrProducer<T>  {
//!     type Output = T;
//!
//!     fn produce(&self, data: &str) -> Self::Output {
//!         data.parse().ok().unwrap()
//!     }
//! }
//! ```
//!
//! However, this fails to compile, because the parameter `T` is never used.
//!
//! ```text
//! error[E0392]: parameter `T` is never used
//!   --> examples/producer.rs
//!    |
//!    | struct FromStrProducer<T: FromStr>;
//!    |                        ^ unused parameter
//!    |
//!    = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
//! ```
//!
//! If we add a [`PhantomData<T>`], our `Producer` works.
//!
//! ```
//! use std::marker::PhantomData;
//!
//! struct FromStrProducer<T: FromStr> {
//!     _phantom: PhantomData<T>,
//! }
//! ```
//!
//! Now, let's create a struct `RcString` that wraps an
#![cfg_attr(feature = "std", doc = "<code>[Rc](std::rc::Rc)\\<[String]></code>.")]
#![cfg_attr(not(feature = "std"), doc = "`Rc<String>.`")]
//! `FromStrProducer<RcString>` implements `Producer`.
//!
//! ```
//! use std::convert::Infallible;
//! use std::rc::Rc;
//!
//! #[derive(Eq, PartialEq, Debug)]
//! struct RcString(Rc<String>);
//!
//! impl FromStr for RcString {
//!     type Err = Infallible;
//!
//!     fn from_str(s: &str) -> Result<Self, Self::Err> {
//!         Ok(Self(Rc::new(s.to_owned())))
//!     }
//! }
//!
//! let producer = FromStrProducer {
//!     _phantom: PhantomData
//! };
//!
//! let rc_string: RcString = producer.produce("hello");
//! assert_eq!(rc_string, RcString(Rc::new("hello".to_owned())));
//! ```
//!
//! Now, let's try creating a producer and sending it to another thread.
//!
//! ```
//! use std::thread;
//!
//! let producer = ProducesFive;
//!
//! thread::spawn(move || {
//!     assert_eq!(producer.produce("a"), 5);
//! })
//! .join()
//! .unwrap();
//! ```
//!
//! Sending `ProducesFive` works, but what happens if we try to
//! send `FromStrProducer<RcString>`?
//!
//! ```compile_fail
//! let producer = FromStrProducer::<RcString> {
//!     _phantom: PhantomData
//! };
//!
//! thread::spawn(move || {
//!     assert_eq!(producer.produce("a"), RcString(Rc::new("a".to_owned())));
//! })
//! .join()
//! .unwrap();
//! ```
//!
//! We get a compilation error, because `RcString` is not [`Send`].
//!
//! ```text
//! error[E0277]: `Rc<String>` cannot be sent between threads safely
//!
//! ...
//!
//! note: required because it appears within the type `RcString`
//!    --> examples/producer.rs
//!     |
//!     | struct RcString(Rc<String>);
//!     |        ^^^^^^^^
//!     = note: required because it appears within the type `PhantomData<RcString>`
//! note: required because it appears within the type `FromStrProducer<RcString>`
//!
//! ...
//! ```
//!
//! `RcString` is required to be [`Send`] despite the fact that we are never
//! actually sending any `RcString`s between threads. The issue comes from the
//! fact that the [`PhantomData`] makes `FromStrProducer` act like it owns a
//! `RcString` when it doesn't, making `FromStrProducer<RcString>` not
//! [`Send`].
//!
//! This is where [`unused`](./index.html) comes in.
//! `FromStrProducer<RcString>` is [`Send`] if we replace [`PhantomData`] with
//! [`Unused`].
//!
//! ```
//! use unused::Unused;
//!
//! struct FromStrProducer<T: FromStr> {
//!     _unused: Unused<T>,
//! }
//! ```
//!
//! Now, sending `FromStrProducer<RcString>` between threads works.
//!
//! ```
//! let producer = FromStrProducer::<RcString> {
//!     _unused: Unused::new(),
//! };
//!
//! thread::spawn(move || {
//!     assert_eq!(producer.produce("a"), RcString(Rc::new("a".to_owned())));
//! })
//! .join()
//! .unwrap();
//! ```
//!
//! [`Unused`] can be used instead of [`PhantomData`] when a type has unused
//! generic parameters that are not conceptually owned by the type.

#![cfg_attr(not(feature = "std"), no_std)]

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::panic::RefUnwindSafe;
#[cfg(feature = "std")]
use std::panic::UnwindSafe;

/// A struct that allows types to have unused generic parameters that do not act
/// like they are owned.
///
/// See the [crate documentation](./index.html) for more.
pub struct Unused<T: ?Sized>(PhantomData<*const T>);

impl<T: ?Sized> Unused<T> {
    /// Creates a new [`Unused<T>`].
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

/// Safety: [`Unused<T>`] can be sent between threads regardless of whether `T`
/// can. It does not store or access any state.
unsafe impl<T: ?Sized> Send for Unused<T> {}

/// Safety: [`Unused<T>`] can be shared between threads regardless of whether
/// `T` can. It does not store or access any state.
unsafe impl<T: ?Sized> Sync for Unused<T> {}

#[cfg(feature = "std")]
/// [`Unused`] has no invariants that can be broken.
impl<T: ?Sized> UnwindSafe for Unused<T> {}

#[cfg(feature = "std")]
/// [`Unused`] has no invariants that can be broken.
impl<T: ?Sized> RefUnwindSafe for Unused<T> {}

impl<T: ?Sized> Clone for Unused<T> {
    fn clone(&self) -> Self {
        // All instances of `Unused` are the same, so `Unused::new()` will produce the
        // same thing.
        Self::new()
    }

    // This does nothing, since all instances of `Unused` are the same.
    fn clone_from(&mut self, _source: &Self) {}
}

// `Unused` has no state or special semantics. All instances of it are the same.
impl<T: ?Sized> Copy for Unused<T> {}

impl<T: ?Sized> Default for Unused<T> {
    fn default() -> Self {
        // This default is the same as any instance.
        Self::new()
    }
}

impl<T: ?Sized> fmt::Debug for Unused<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Unused").field(&self.0).finish()
    }
}

// This `Hash` implementation does nothing, because all instances of `Unused`
// are the same.
impl<T: ?Sized> Hash for Unused<T> {
    fn hash<H>(&self, _state: &mut H)
    where
        H: Hasher,
    {
    }

    fn hash_slice<H>(_data: &[Self], _state: &mut H)
    where
        H: Hasher,
    {
    }
}

// All `Unused`s are the same as one another, and this implementation reflects
// that.
impl<T: ?Sized> PartialEq for Unused<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

// The equality is reflexive, symmetric, and transitive.
impl<T: ?Sized> Eq for Unused<T> {}

// All `Unused`s are equal to one another, and this implementation reflects
// that.
impl<T: ?Sized> PartialOrd for Unused<T> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }

    fn lt(&self, _other: &Self) -> bool {
        false
    }

    fn le(&self, _other: &Self) -> bool {
        true
    }

    fn gt(&self, _other: &Self) -> bool {
        false
    }

    fn ge(&self, _other: &Self) -> bool {
        true
    }
}

// All `Unused`s are equal to one another, and this implementation reflects
// that. The ordering is total, asymmetric, and transitive
impl<T: ?Sized> Ord for Unused<T> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }

    // It does not matter which side is returned, because both are the same.
    fn max(self, _other: Self) -> Self {
        self
    }

    // It does not matter which side is returned, because both are the same.
    fn min(self, _other: Self) -> Self {
        self
    }

    // It does not matter what is returned, because all are the same.
    fn clamp(self, _min: Self, _max: Self) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::Unused;

    #[test]
    fn debug_impl() {
        assert_eq!(
            std::format!("{:?}", Unused::<usize>::new()),
            "Unused(PhantomData)"
        );
    }
}
