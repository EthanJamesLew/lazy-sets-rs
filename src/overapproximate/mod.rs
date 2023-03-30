/*!
 * overapproximation algorithm for convex sets.
 */
use minilp::{ComparisonOp, OptimizationDirection, Problem, Variable};
use nalgebra::{DMatrix, DVector, RealField, SVector};
use num_traits::{FromPrimitive, ToPrimitive, Zero};
use rand::{distributions::uniform::SampleUniform, Rng};

use crate::convex::{DHalfspacePolytope, SupportFunction};

/// determines for a polyhedral set Ax <= b, determine if a new constraint c^T x <= d is redundant
fn is_constraint_redundant<N, const D: usize>(
    faces: &Vec<SVector<N, D>>,
    upper_bounds: &Vec<N>,
    new_face: SVector<N, D>,
    new_upper_bound: N,
) -> bool
where
    N: RealField + ToPrimitive,
{
    // let A = faces, b = upper_bounds, c = new_face, d = new_upper_bound
    // solve max c^T x st
    // Ax <= b
    // c^T x <= d

    let mut problem = Problem::new(OptimizationDirection::Maximize);

    // make variables and multiply be new_face
    let mut vars = Vec::new();
    for i in 0..new_face.len() {
        vars.push(problem.add_var(
            new_face[i].to_f64().unwrap(),
            (f64::NEG_INFINITY, f64::INFINITY),
        ));
    }

    // add constraints from faces and upper_bounds
    for i in 0..faces.len() {
        let mut row = Vec::<(Variable, f64)>::new();
        for j in 0..faces[i].len() {
            row.push((vars[j], faces[i][j].to_f64().unwrap()));
        }
        // convert row into list
        problem.add_constraint(
            row.as_slice(),
            ComparisonOp::Le,
            upper_bounds[i].to_f64().unwrap(),
        );
    }

    // add new_face * x <= new_upper_bound + 1
    let mut row = Vec::<(Variable, f64)>::new();
    for i in 0..new_face.len() {
        row.push((vars[i], new_face[i].to_f64().unwrap()));
    }
    problem.add_constraint(
        row.as_slice(),
        ComparisonOp::Le,
        new_upper_bound.to_f64().unwrap() + 1.0,
    );

    let solution = problem.solve().unwrap();
    let objective = N::from_f64(solution.objective()).unwrap();

    return objective <= new_upper_bound;
}

/// overapproximate a convex set with a dynamically allocated H polytope
pub fn overapproximate<N, const D: usize>(
    convex_set: &dyn SupportFunction<N, D>,
) -> DHalfspacePolytope<N>
where
    N: RealField + Copy + FromPrimitive + ToPrimitive + SampleUniform,
{
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

        // compute sigma and rho
        let (_, sigma) = convex_set.support(&v);

        // get new upper bound
        let b = sigma.dot(&v);

        if !is_constraint_redundant::<N, D>(&unit_vectors, &bs, v, b) {
            bs.push(b);
            unit_vectors.push(v);
        }
    }

    // flatten unit_vectors in column major order
    let mut unit_vectors_cm = Vec::new();
    for i in 0..D {
        for j in 0..unit_vectors.len() {
            unit_vectors_cm.push(unit_vectors[j][i]);
        }
    }

    let a_transform = DMatrix::<N>::from_vec(unit_vectors.len(), D, unit_vectors_cm);
    let b = DVector::<N>::from_vec(bs);

    // create a dynamic polytope with vn and bs
    DHalfspacePolytope::<N>::new(a_transform, b)
}
