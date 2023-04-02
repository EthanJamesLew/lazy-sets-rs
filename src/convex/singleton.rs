/*!
 * A singleton.
 */
use nalgebra::{RealField, SVector};

use super::LazySet;

/// A singleton.
#[derive(Clone)]
pub struct Singleton<N, const D: usize> {
    /// The point.
    pub point: SVector<N, D>,
}

impl<N, const D: usize> Singleton<N, D>
where
    N: RealField,
{
    /// Create a new singleton.
    pub fn new(point: SVector<N, D>) -> Singleton<N, D> {
        Singleton { point }
    }
}

impl<N, const D: usize> LazySet<N, D> for Singleton<N, D>
where
    N: RealField,
{
    fn support(&self, direction: &SVector<N, D>) -> (N, SVector<N, D>) {
        (direction.dot(&self.point), self.point)
    }
}
