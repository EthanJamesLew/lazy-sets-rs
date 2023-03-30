/*!
 * Operations on geometric domains.
 */
pub mod hull;
pub mod minkowski;
pub mod transform;

pub use hull::ConvexHull;
pub use minkowski::MinkowskiSum;
pub use transform::LinearTransformation;
