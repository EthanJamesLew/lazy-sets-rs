use nalgebra::SVector;

/// A support function is a function that returns the support point of a convex set in a given direction.
pub trait SupportFunction<N, const D: usize> {
    fn support(&self, direction: &SVector<N, D>) -> (N, SVector<N, D>);
}
