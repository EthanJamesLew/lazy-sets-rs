use nalgebra::{RealField, SVector};

use crate::convex::SupportFunction;

/// Convex hull of two convex sets.
pub struct ConvexHull<N, const D: usize> {
    /// The first support function.
    sf1: Box<dyn SupportFunction<N, D>>,
    /// The second support function.
    sf2: Box<dyn SupportFunction<N, D>>,
}

impl<N, const D: usize> ConvexHull<N, D> {
    /// Create a new convex hull of two convex sets.
    pub fn new(
        sf1: Box<dyn SupportFunction<N, D>>,
        sf2: Box<dyn SupportFunction<N, D>>,
    ) -> ConvexHull<N, D> {
        ConvexHull { sf1, sf2 }
    }
}

impl<N, const D: usize> SupportFunction<N, D> for ConvexHull<N, D>
where
    N: RealField,
{
    fn support(&self, direction: &SVector<N, D>) -> (N, SVector<N, D>) {
        let (d1, p1) = self.sf1.support(direction);
        let (d2, p2) = self.sf2.support(direction);
        if d1 > d2 {
            (d1, p1)
        } else {
            (d2, p2)
        }
    }
}
