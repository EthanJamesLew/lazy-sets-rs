use nalgebra::{RealField, SVector};

use crate::convex::SupportFunction;

/// A Minkowski sum of two convex sets.
/// The Minkowski sum of two convex sets A and B is the set of all possible sums a + b where a ∈ A and b ∈ B.
pub struct MinkowskiSum<N, const D: usize> {
    /// The first support function.
    sf1: Box<dyn SupportFunction<N, D>>,
    ///  The second support function.
    sf2: Box<dyn SupportFunction<N, D>>,
}

impl<N, const D: usize> MinkowskiSum<N, D> {
    pub fn new(
        sf1: Box<dyn SupportFunction<N, D>>,
        sf2: Box<dyn SupportFunction<N, D>>,
    ) -> MinkowskiSum<N, D> {
        MinkowskiSum { sf1, sf2 }
    }
}

impl<N, const D: usize> SupportFunction<N, D> for MinkowskiSum<N, D>
where
    N: RealField,
{
    fn support(&self, direction: &SVector<N, D>) -> (N, SVector<N, D>) {
        let (d1, p1) = self.sf1.support(direction);
        let (d2, p2) = self.sf2.support(direction);
        (d1 + d2, p1 + p2)
    }
}
