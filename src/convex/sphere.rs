/*!
 * HyperSphere
 */

use super::traits::SupportFunction;
use nalgebra::{RealField, SVector};

/// A hyper-sphere.
#[derive(Clone)]
pub struct Hypersphere<N, const D: usize> {
    /// The radius of the sphere.
    pub radius: N,
}

impl<N, const D: usize> Hypersphere<N, D>
where
    N: RealField,
{
    /// Create a new sphere.
    pub fn new(radius: N) -> Hypersphere<N, D> {
        Hypersphere { radius }
    }
}

impl<N, const D: usize> SupportFunction<N, D> for Hypersphere<N, D>
where
    N: RealField,
{
    fn support(&self, direction: &SVector<N, D>) -> (N, SVector<N, D>) {
        let d = direction.normalize();
        let y = d.scale(self.radius);
        (self.radius * d.norm(), y)
    }
}
