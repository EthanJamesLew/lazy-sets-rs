use nalgebra::{RealField, SMatrix, SVector};

use crate::convex::SupportFunction;

pub struct LinearTransformation<N, const D: usize> {
    /// The linear transformation matrix.
    a_transform: SMatrix<N, D, D>,
    /// The support function.
    sf: Box<dyn SupportFunction<N, D>>,
}

impl<N, const D: usize> LinearTransformation<N, D> {
    pub fn new(
        a_transform: SMatrix<N, D, D>,
        sf: Box<dyn SupportFunction<N, D>>,
    ) -> LinearTransformation<N, D> {
        LinearTransformation { a_transform, sf }
    }
}

impl<N, const D: usize> SupportFunction<N, D> for LinearTransformation<N, D>
where
    N: RealField,
{
    fn support(&self, direction: &SVector<N, D>) -> (N, SVector<N, D>) {
        let (d, p) = self.sf.support(&(self.a_transform.transpose() * direction));
        (d, self.a_transform * p)
    }
}
