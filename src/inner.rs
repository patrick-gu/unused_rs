use crate::End;

/// Represents a type that can be used as a `T` for
/// [`Unused<T>`](type@crate::Unused).
///
/// The `inconstruable` method ensures that implementers can never be
/// constructed.
///
/// When implemented for [`Invariant`](crate::Invariant),
/// [`Covariant`](crate::Covariant), or [`Contravariant`](crate::Contravariant),
/// the type acts a container for a `T`, as well as another `UnusedInner`,
/// which allows for chaining.
///
/// When implemented for [`End`](crate::End), the type ends the chain of
/// `UnusedInner`s.
///
/// This trait is sealed.
pub trait UnusedInner: Sized + Copy + Send + Sync + Unpin {
    fn inconstruable(self) -> End;
}
