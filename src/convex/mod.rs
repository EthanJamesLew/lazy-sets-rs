/*!
* Convex Sets
*/
pub mod halfspace;
pub mod singleton;
pub mod sphere;
mod traits;

pub use halfspace::{DHalfspacePolytope, SHalfspacePolytope};
pub use singleton::Singleton;
pub use sphere::Hypersphere;
pub use traits::{DSupportFunction, SupportFunction};
