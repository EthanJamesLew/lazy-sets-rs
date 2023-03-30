/*!
* Convex Sets
*/
pub mod halfspace;
pub mod sphere;
mod traits;

pub use halfspace::{DHalfspacePolytope, SHalfspacePolytope};
pub use sphere::Hypersphere;
pub use traits::SupportFunction;
