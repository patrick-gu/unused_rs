# Unused

## About Unused

The `unused` crate provides `Unused`, a struct that
allows types to have unused generic parameters that do not act like they
are owned.

Unlike `PhantomData<T>`, `Unused<T>` does not tell the compiler that
the type owns a `T`.

Because `Unused<T>` does not "own" a `T`, it is `Send`, `Sync`, and
`Unpin` even if `T` is not. It is just a container for the `T` type, and
has no data or state of its own. All instances of it are the same.

# Feedback
If you experience any issues or have any feedback, please feel free to open
an issue on the
[GitHub repository](https://github.com/patrick-gu/unused_rs/issues/new).

## `no_std` Support

`unused` optionally supports `no_std`. Disable the default
crate features to enable this.

## Example

Consider a trait `Producer`.

We implement `Producer` for a struct `ProducesFive`, and for `usize`.

```rust
trait Producer {
    type Output;

    fn produce(&self, data: &str) -> Self::Output;
}

struct ProducesFive;

impl Producer for ProducesFive {
    type Output = i32;

    fn produce(&self, _data: &str) -> Self::Output {
        5
    }
}

impl Producer for usize {
    type Output = usize;

    fn produce(&self, _data: &str) -> Self::Output {
        *self
    }
}
```

Now, let's say that we want to have a `Producer` for any type that
implements `FromStr` by parsing the provided `data`.
We create a `FromStrProducer` struct, to not conflict with the existing
implementation for `usize`.

```rust
use std::str::FromStr;

struct FromStrProducer<T: FromStr>;

impl<T: FromStr> Producer for FromStrProducer<T>  {
    type Output = T;

    fn produce(&self, data: &str) -> Self::Output {
        data.parse().ok().unwrap()
    }
}
```

However, this fails to compile, because the parameter `T` is never used.

```text
error[E0392]: parameter `T` is never used
  --> examples/producer.rs
   |
   | struct FromStrProducer<T: FromStr>;
   |                        ^ unused parameter
   |
   = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
```

If we add a `PhantomData<T>`, our `Producer` works.

```rust
use std::marker::PhantomData;

struct FromStrProducer<T: FromStr> {
    _phantom: PhantomData<T>,
}
```

Now, let's create a struct `RcString` that wraps an
`Rc<String>`.
`FromStrProducer<RcString>` implements `Producer`.

```rust
use std::convert::Infallible;
use std::rc::Rc;

#[derive(Eq, PartialEq, Debug)]
struct RcString(Rc<String>);

impl FromStr for RcString {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Rc::new(s.to_owned())))
    }
}

let producer = FromStrProducer {
    _phantom: PhantomData
};

let rc_string: RcString = producer.produce("hello");
assert_eq!(rc_string, RcString(Rc::new("hello".to_owned())));
```

Now, let's try creating a producer and sending it to another thread.

```rust
use std::thread;

let producer = ProducesFive;

thread::spawn(move || {
    assert_eq!(producer.produce("a"), 5);
})
.join()
.unwrap();
```

Sending `ProducesFive` works, but what happens if we try to
send `FromStrProducer<RcString>`?

```rust
let producer = FromStrProducer::<RcString> {
    _phantom: PhantomData
};

thread::spawn(move || {
    assert_eq!(producer.produce("a"), RcString(Rc::new("a".to_owned())));
})
.join()
.unwrap();
```

We get a compilation error, because `RcString` is not [`Send`].

```text
error[E0277]: `Rc<String>` cannot be sent between threads safely

...

note: required because it appears within the type `RcString`
   --> examples/producer.rs
    |
    | struct RcString(Rc<String>);
    |        ^^^^^^^^
    = note: required because it appears within the type `PhantomData<RcString>`
note: required because it appears within the type `FromStrProducer<RcString>`

...
```

`RcString` is required to be `Send` despite the fact that we are never
actually sending any `RcString`s between threads. The issue comes from the
fact that the `PhantomData` makes `FromStrProducer` act like it owns a
`RcString` when it doesn't, making `FromStrProducer<RcString>` not
`Send`.

This is where `unused` comes in.
`FromStrProducer<RcString>` is `Send` if we replace `PhantomData` with
`Unused`.

```rust
use unused::Unused;

struct FromStrProducer<T: FromStr> {
    _unused: Unused<T>,
}
```

Now, sending `FromStrProducer<RcString>` between threads works.

```rust
let producer = FromStrProducer::<RcString> {
    _unused: Unused::new(),
};

thread::spawn(move || {
    assert_eq!(producer.produce("a"), RcString(Rc::new("a".to_owned())));
})
.join()
.unwrap();
```

`Unused` can be used instead of `PhantomData` when a type has unused
generic parameters that are not conceptually owned by the type.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

