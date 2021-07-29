# Unused

## About Unused

The `unused` crate allows types to have unused generic parameters that do
not act like they are owned.

## Example

Imagine we have a struct `LazyFromStr`, which contains a `&'static str` and can
lazily create a `T` using its `FromStr` impl.

To have `T` be a generic parameter of `LazyFromStr`, we can use a
`PhantomData`. Otherwise, we get a
compilation error that the parameter `T` is never used.

```rust
use std::marker::PhantomData;
use std::str::FromStr;

struct LazyFromStr<T> {
    str: &'static str,
    phantom: PhantomData<T>,
}

impl<T: FromStr> LazyFromStr<T> {
    fn create(&self) -> T {
        match T::from_str(self.str) {
            Ok(t) => t,
            Err(_) => panic!(),
        }
    }
}
```

The issue with using `PhantomData` is that
`LazyFromStr<T>` is only `Send` and `Sync` if `T` also is, even though
our `LazyFromStr<T>` does not own a `T`.

This is where `unused` comes in.

```rust
// We need to import `Unused`.
use unused::Unused;

struct LazyFromStr<T> {
    str: &'static str,
    // Use the `Unused` macro instead of `PhantomData`.
    unused: Unused!(T),
}

use std::convert::Infallible;
use std::rc::Rc;

// `RcString` is not `Send` or `Sync`.
struct RcString(Rc<String>);

impl FromStr for RcString {
    type Err = Infallible;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(Self(Rc::new(str.to_owned())))
    }
}

let lazy: LazyFromStr<RcString> = LazyFromStr {
    str: "a",
    // Use `Unused` as a value.
    unused: Unused,
};

use std::thread;

// `lazy` is `Send` (even though `RcString` is not), so we can send it between threads.
thread::spawn(move ||{
    let _ = lazy.create();
})
.join()
.unwrap();
```

## Usage

First, add `unused` to your `dependencies` in `Cargo.toml`:

```toml
unused = "0.1"
```

Create a simple struct with an unused generic parameter:

```rust
use unused::Unused;

struct Foo<T> {
    some_string: String,
    unused: Unused!(T),
}

let foo: Foo<usize> = Foo {
    some_string: "hello".to_owned(),
    unused: Unused,
};
```

See the [docs](https://docs.rs/unused) for the full documentation.

## Feedback

If you experience any issues or have any feedback, please feel free to open
an issue on the
[GitHub repository](https://github.com/patrick-gu/unused_rs/issues/new).

## Related/Similar Crates

-   [type_variance](https://crates.io/crates/type-variance) - Marker traits for
    subtype variance
-   [ghost](https://crates.io/crates/ghost) - Define your own PhantomData
-   [rich-phantoms](https://crates.io/crates/rich-phantoms) - Phantom types with
    control over variance and sync/sync inheritance

## License

Licensed under either of

-   Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
    http://www.apache.org/licenses/LICENSE-2.0)
-   MIT license
    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
