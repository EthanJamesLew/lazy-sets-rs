use std::time::Instant;

use lazy_reach::{
    convex::SHalfspacePolytope, convex::SupportFunction, operation::LinearTransformation,
    operation::MinkowskiSum, overapproximate,
};
use nalgebra::{SMatrix, SVector};

const DIM: usize = 2;
type Float = f32;

fn main() {
    // make a polytope
    // create 4x2 H matrix
    //let a_transform = DMatrix::from_vec(4, DIM, vec![1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0]);

    //let h = DVector::from_vec(vec![2.0, 2.0, 2.0, 2.0]);

    //println!("H {}", a_transform);
    //let polytope = DHalfspacePolytope::<Float>::new(a_transform, h);

    let a_transform =
        SMatrix::<Float, 4, 2>::from_vec(vec![1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0]);
    let h = SVector::<Float, 4>::from_vec(vec![2.0, 2.0, 2.0, 2.0]);
    let polytope = SHalfspacePolytope::<Float, 4, 2>::new(a_transform, h);

    // create minkowski sum
    let ms =
        MinkowskiSum::<Float, DIM>::new(Box::new(polytope.clone()), Box::new(polytope.clone()));
    // sum with another ms
    let ms2 = MinkowskiSum::<Float, DIM>::new(Box::new(ms), Box::new(polytope.clone()));

    // make a rotation smatrix sin/cos
    let rot = SMatrix::<Float, 2, 2>::from_vec(vec![
        Float::cos(0.2),
        -Float::sin(0.2),
        Float::sin(0.2),
        Float::cos(0.2),
    ]);

    let lt = LinearTransformation::<Float, DIM>::new(rot, Box::new(ms2));

    let now = Instant::now();

    let (obj, supp) =
        SupportFunction::<Float, DIM>::support(&lt, &SVector::from_vec(vec![-1.0, 1.0]));

    let elapsed_time = now.elapsed();

    let oa = overapproximate::overapproximate::<Float, DIM>(&lt);

    // print the oa a_tranform and upperbounds as csv
    for i in 0..oa.a_transform.nrows() {
        for j in 0..oa.a_transform.ncols() {
            print!("{},", oa.a_transform[(i, j)]);
        }
        print!("{}", oa.upper_bounds[i]);
        println!();
    }

    //println!("obj: {}", obj);
    //println!("supp: {}", supp);
    //println!("overapproximate: {}", oa.a_transform);
    //println!("elapsed time: {}ns", elapsed_time.as_nanos());
}
