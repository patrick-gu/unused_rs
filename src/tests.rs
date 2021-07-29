extern crate std;

use std::cell::UnsafeCell;
use std::marker::PhantomPinned;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::prelude::v1::*;
use std::rc::Rc;

use crate::Unused;

#[test]
fn variance() {
    fn _simple_covariant<'a>(
        unused: Unused!(&'static str: covariant),
    ) -> Unused!(&'a str: covariant) {
        unused
    }

    fn _covariant_and_invariant<'a>(
        unused: Unused!(u8: invariant, &'static u16: covariant, &'a str: invariant),
    ) -> Unused!(u8: invariant, &'a u16: covariant, &'a str: invariant) {
        unused
    }

    fn _covariant_and_contravariant<'a>(
        unused: Unused!(&'a str: contravariant, &'static str: covariant,),
    ) -> Unused!(&'static str: contravariant, &'a str: covariant) {
        unused
    }
}

#[test]
fn auto_traits() {
    fn auto_traits_are_implemented<T: Send + Sync + Unpin + UnwindSafe + RefUnwindSafe>() {}

    auto_traits_are_implemented::<
        Unused!(Rc<str>: covariant, UnsafeCell<i32>, PhantomPinned: contravariant,),
    >()
}

#[test]
fn macro_used_as_value() {
    struct Foo<T> {
        #[allow(dead_code)]
        unused: Unused!(T),
    }

    let _ = Foo {
        unused: Unused!(u8),
    };
}

/// ```compile_fail
/// use unused::Unused;
///
/// fn invariant<'a>(
///     unused: Unused!(&'static str),
/// ) -> Unused!(&'a str) {
///     unused
/// }
/// ```
pub fn _invariant_not_covariant() {}

/// ```compile_fail
/// use unused::Unused;
///
/// fn invariant<'a>(
///     unused: Unused!(&'a str: invariant),
/// ) -> Unused!(&'static str: invariant) {
///     unused
/// }
/// ```
pub fn _invariant_not_contravariant() {}
