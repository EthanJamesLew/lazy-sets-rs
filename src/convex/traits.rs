use nalgebra::{DVector, SVector};

/// A support function is a function that returns the support point of a convex set in a given direction.
pub trait LazySet<N, const D: usize> {
    /// Returns the support function and support point of the convex set in the given direction.
    /// They are done together for performance and simplicity.
    fn support(&self, direction: &SVector<N, D>) -> (N, SVector<N, D>);
}

/// A support function is a function that returns the support point of a convex set in a given direction.
pub trait DLazySet<N> {
    /// Returns the support function and support point of the convex set in the given direction.
    /// They are done together for performance and simplicity.
    fn support(&self, direction: &DVector<N>) -> (N, DVector<N>);
}
