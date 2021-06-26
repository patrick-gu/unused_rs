use unused::Unused;

// Instead of this:
struct MultipleUnused<T, U> {
    _unused_t: Unused<T>,
    _unused_u: Unused<U>,
}

// You can instead use this:
struct SingleUnused<T, U> {
    _unused: Unused<(T, U)>,
}

fn main() {
    let _multiple = MultipleUnused::<usize, i32> {
        _unused_t: Unused::new(),
        _unused_u: Unused::new(),
    };

    let _single = SingleUnused::<usize, i32> {
        _unused: Unused::new(),
    };
}
