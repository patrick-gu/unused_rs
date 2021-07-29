//! # About Unused
//!
//! The `unused` crate allows types to have unused generic parameters that do
//! not act like they are owned.
//!
//! ## Feedback
//!
//! If you experience any issues or have any feedback, please feel free to open
//! an issue on the
//! [GitHub repository](https://github.com/patrick-gu/unused_rs/issues/new).
//!
//! ## `no_std` Support
//!
//! `unused` supports `no_std`.
//!
//! ## Example
//!
//! Imagine we have a struct `LazyFromStr`, which contains a <code>&'static [str]</code> and can
//! lazily create a `T` using its [`FromStr`](core::str::FromStr) impl.
//!
//! To have `T` be a generic parameter of `LazyFromStr`, we can use a
//! [`PhantomData`](core::marker::PhantomData). Otherwise, we get a
//! compilation error that the parameter `T` is never used.
//!
//! ```
//! use std::marker::PhantomData;
//! use std::str::FromStr;
//!
//! struct LazyFromStr<T> {
//!     str: &'static str,
//!     phantom: PhantomData<T>,
//! }
//!
//! impl<T: FromStr> LazyFromStr<T> {
//!     fn create(&self) -> T {
//!         match T::from_str(self.str) {
//!             Ok(t) => t,
//!             Err(_) => panic!(),
//!         }
//!     }
//! }
//! ```
//!
//! The issue with this is that `LazyFromStr<T>` is only [`Send`] and [`Sync`]
//! if `T` also is.
//!
//! This is where `unused` comes in.
//!
//! ```
//! // We need to import `Unused`.
//! use unused::Unused;
//!
//! struct LazyFromStr<T> {
//!     str: &'static str,
//!     // Use the `Unused` macro instead of `PhantomData`.
//!     unused: Unused!(T),
//! }
//! # use std::str::FromStr;
//! # impl<T: FromStr> LazyFromStr<T> {
//! #     fn create(&self) -> T {
//! #         match T::from_str(self.str) {
//! #             Ok(t) => t,
//! #             Err(_) => panic!(),
//! #         }
//! #     }
//! # }
//!
//! use std::convert::Infallible;
//! use std::rc::Rc;
//!
//! // `RcString` is not `Send` or `Sync`.
//! struct RcString(Rc<String>);
//!
//! impl FromStr for RcString {
//!     type Err = Infallible;
//!
//!     fn from_str(str: &str) -> Result<Self, Self::Err> {
//!         Ok(Self(Rc::new(str.to_owned())))
//!     }
//! }
//!
//! let lazy: LazyFromStr<RcString> = LazyFromStr {
//!     str: "a",
//!     // Use `Unused` as a value.
//!     unused: Unused,
//! };
//!
//! use std::thread;
//!
//! // `lazy` is `Send` (even though `RcString` is not), so we can send it between threads.
//! thread::spawn(move ||{
//!     let _ = lazy.create();
//! })
//! .join()
//! .unwrap();
//! ```
//!
//! [By default, `Unused` makes generics invariant](Unused!#variances).
//!
//! See the [`Unused!`] macro for more examples.

#![no_std]

mod contravariant;
mod covariant;
mod end;
mod inner;
mod invariant;
#[cfg(test)]
mod tests;
mod unused;

#[doc(hidden)]
pub use crate::contravariant::Contravariant;
#[doc(hidden)]
pub use crate::covariant::Covariant;
#[doc(hidden)]
pub use crate::end::End;
#[doc(hidden)]
pub use crate::invariant::Invariant;
use crate::unused::UnusedImpl;
#[doc(hidden)]
pub use crate::unused::UnusedImpl::*;

/// A container for unused generic types.
///
/// The `Unused` type can be created using the [`Unused!`] macro.
///
/// `Unused` can also be used as a value for any type `Unused<T>`:
///
/// ```
/// use unused::Unused;
///
/// let _: Unused!(usize) = Unused;
/// ```
///
/// See the [crate documentation](crate) for more information.
pub type Unused<T> = UnusedImpl<T>;

/// A macro that allows for the creation of [`type@Unused`] type containers.
///
/// A basic example of usage can be found in the [crate documentation](crate).
///
/// ## Usage
///
/// ```
/// use unused::Unused;
///
/// struct Foo<A, B, C, D, E> {
///     unused: Unused!(
///         A,
///         B,
///         C: covariant,
///         D: contravariant,
///         E: invariant,
///     ),
/// }
///
/// ```
///
/// ## Variances
///
/// You can have different
/// [type variances](https://doc.rust-lang.org/reference/subtyping.html) when
/// working with `Unused`.
///
/// ### Invariant
///
/// Invariance is the default:
///
/// ```
/// # use unused::Unused;
/// let unused: Unused!(u8) = Unused;
/// // is the same as:
/// let unused: Unused!(u8: invariant) = Unused;
/// ```
///
/// ### Covariance and Contravariance
///
/// To make `Foo` covariant over `A` and contravariant over `B`:
///
/// ```
/// # use unused::Unused;
/// struct Foo<A, B> {
///     unused: Unused!(A: covariant, B: contravariant),
/// }
/// ```
///
/// ### Lifetimes
///
/// Variance is particularily useful when it comes to lifetimes:
///
/// ```
/// # use unused::Unused;
/// struct Foo<'foo> {
///     unused: Unused!(&'foo (): covariant),   
/// }
///
/// fn change_foo_lifetime<'a>(foo: Foo<'static>) -> Foo<'a> {
///     foo
/// }
/// ```
#[macro_export]
macro_rules! Unused {
    ($($type:ty $(: $variance:ident)?),+ $(,)?) => {
        $crate::Unused::<$crate::__impl_Unused!($($type $(:$variance)?,)+)>
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_Unused {
    () => {
        $crate::End
    };
    ($type:ty, $($types:ty $(:$variances:ident)?,)*) => {
        $crate::Invariant::<$type, $crate::__impl_Unused!($($types $(: $variances)?,)*)>
    };
    ($type:ty: invariant, $($types:ty $(: $variances:ident)?,)*) => {
        $crate::Invariant::<$type, $crate::__impl_Unused!($($types $(: $variances)?,)*)>
    };
    ($type:ty: covariant, $($types:ty $(: $variances:ident)?,)*) => {
        $crate::Covariant::<$type, $crate::__impl_Unused!($($types $(: $variances)?,)*)>
    };
    ($type:ty: contravariant, $($types:ty $(: $variances:ident)?,)*) => {
        $crate::Contravariant::<$type, $crate::__impl_Unused!($($types $(: $variances)?,)*)>
    };
}
