use std::sync::Arc;

use nalgebra::{SVector, RealField, DMatrix, DVector};
use num_traits::{Zero, Float, FromPrimitive};
use rand::{Rng, distributions::uniform::SampleUniform};

use crate::convex::{SupportFunction, DHalfspacePolytope};

pub fn overapproximate<N, const D: usize>(convex_set: &dyn SupportFunction<N, D>) -> DHalfspacePolytope<N> 
where N: RealField + Copy + FromPrimitive + SampleUniform {
    // generate random unit vectors pointing uniformly in the unit sphere
    let num_samples = 600;
    let mut rng = rand::thread_rng();
    let mut unit_vectors = Vec::new();
    // compute the support function of the convex set for each unit vector
    let mut bs = Vec::new();
    for _ in 0..num_samples {
        // create a random vector
        let mut v = SVector::<N, D>::zero();
        for i in 0..D {
            v[i] = rng.gen_range(N::from_f64(-1.0).unwrap()..N::from_f64(1.0).unwrap());
        }
        v = v.normalize();
        unit_vectors.push(v);
        
        // compute sigma and rho
        let (rho, sigma) = convex_set.support(&v);

        // push bs
        let b = sigma.dot(&v);
        bs.push(b);
    }

    // flatten unit_vectors in column major order
    let mut unit_vectors_cm = Vec::new();
    for i in 0..D {
        for j in 0..num_samples {
            unit_vectors_cm.push(unit_vectors[j][i]);
        }
    }

    let H = DMatrix::<N>::from_vec(num_samples, D, unit_vectors_cm);
    let b = DVector::<N>::from_vec(bs);

    // create a dynamic polytope with vn and bs
    DHalfspacePolytope::<N>::new(H, b)
}
