/*!
 * Hull of Sets Operations
 */
use nalgebra::{RealField, SVector};

use crate::convex::LazySet;

/// Convex hull of two convex sets.
pub struct ConvexHull<N, const D: usize> {
    /// The first support function (left hand side).
    lhs: Box<dyn LazySet<N, D>>,
    /// The second support function (right hand side).
    rhs: Box<dyn LazySet<N, D>>,
}

impl<N, const D: usize> ConvexHull<N, D> {
    /// Create a new convex hull of two convex sets.
    pub fn new(
        lhs: Box<dyn LazySet<N, D>>,
        rhs: Box<dyn LazySet<N, D>>,
    ) -> ConvexHull<N, D> {
        ConvexHull { lhs, rhs }
    }
}

impl<N, const D: usize> LazySet<N, D> for ConvexHull<N, D>
where
    N: RealField,
{
    fn support(&self, direction: &SVector<N, D>) -> (N, SVector<N, D>) {
        let (d1, p1) = self.lhs.support(direction);
        let (d2, p2) = self.rhs.support(direction);
        if d1 > d2 {
            (d1, p1)
        } else {
            (d2, p2)
        }
    }
}
