use minilp::{ComparisonOp, OptimizationDirection, Problem, Variable};
use nalgebra::{DMatrix, DVector, RealField, SMatrix, SVector};
use num_traits::ToPrimitive;

use super::SupportFunction;

#[derive(Clone)]
/// Dynamically allocated Halfspace polytope Hy \le h with a compact solution set
pub struct DHalfspacePolytope<N> {
    /// Hy \le h
    pub a_transform: DMatrix<N>,
    pub upper_bounds: DVector<N>,
}

impl<N> DHalfspacePolytope<N> {
    pub fn new(a_transform: DMatrix<N>, h: DVector<N>) -> DHalfspacePolytope<N> {
        DHalfspacePolytope {
            a_transform,
            upper_bounds: h,
        }
    }
}

impl<N, const D: usize> SupportFunction<N, D> for DHalfspacePolytope<N>
where
    N: RealField + ToPrimitive,
{
    fn support(&self, direction: &SVector<N, D>) -> (N, SVector<N, D>) {
        let mut problem = Problem::new(OptimizationDirection::Maximize);

        // make a variable for every column in H
        let mut vars = Vec::new();
        for i in 0..self.a_transform.ncols() {
            vars.push(problem.add_var(
                direction[i].to_f64().unwrap(),
                (f64::NEG_INFINITY, f64::INFINITY),
            ));
        }

        // add constraints from H and h
        for i in 0..self.a_transform.nrows() {
            let mut row = Vec::<(Variable, f64)>::new();
            for j in 0..self.a_transform.ncols() {
                row.push((vars[j], self.a_transform[(i, j)].to_f64().unwrap()));
            }
            // convert row into list
            problem.add_constraint(
                row.as_slice(),
                ComparisonOp::Le,
                self.upper_bounds[i].to_f64().unwrap(),
            );
        }

        let solution = problem.solve().unwrap();
        let objective = N::from_f64(solution.objective()).unwrap();

        // collect variable values into vec
        let mut values = Vec::<N>::new();
        for i in 0..vars.len() {
            values.push(N::from_f64(solution[vars[i]]).unwrap());
        }

        (objective, SVector::from_vec(values))
    }
}

#[derive(Copy, Clone)]
/// Statically allocated Halfspace polytope Hy \le h with a compact solution set
pub struct SHalfspacePolytope<N, const R: usize, const C: usize> {
    /// Hy \le h
    a_transform: SMatrix<N, R, C>,
    upper_bounds: SVector<N, R>,
}

impl<N, const R: usize, const C: usize> SHalfspacePolytope<N, R, C> {
    pub fn new(a_transform: SMatrix<N, R, C>, h: SVector<N, R>) -> SHalfspacePolytope<N, R, C> {
        SHalfspacePolytope {
            a_transform,
            upper_bounds: h,
        }
    }
}

impl<N, const R: usize, const C: usize> SupportFunction<N, C> for SHalfspacePolytope<N, R, C>
where
    N: RealField + ToPrimitive,
{
    fn support(&self, direction: &SVector<N, C>) -> (N, SVector<N, C>) {
        let mut problem = Problem::new(OptimizationDirection::Maximize);

        // make a variable for every column in H
        let mut vars = Vec::new();
        for i in 0..self.a_transform.ncols() {
            vars.push(problem.add_var(
                direction[i].to_f64().unwrap(),
                (f64::NEG_INFINITY, f64::INFINITY),
            ));
        }

        // add constraints from H and h
        for i in 0..self.a_transform.nrows() {
            let mut row = Vec::<(Variable, f64)>::new();
            for j in 0..self.a_transform.ncols() {
                row.push((vars[j], self.a_transform[(i, j)].to_f64().unwrap()));
            }
            // convert row into list
            problem.add_constraint(
                row.as_slice(),
                ComparisonOp::Le,
                self.upper_bounds[i].to_f64().unwrap(),
            );
        }

        let solution = problem.solve().unwrap();
        let objective = N::from_f64(solution.objective()).unwrap();

        // collect variable values into vec
        let mut values = Vec::<N>::new();
        for i in 0..vars.len() {
            values.push(N::from_f64(solution[vars[i]]).unwrap());
        }

        (objective, SVector::from_vec(values))
    }
}
