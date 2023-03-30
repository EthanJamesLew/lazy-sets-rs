use lazy_reach::{
    convex::SHalfspacePolytope, lazy_operation::LinearTransformation, lazy_operation::MinkowskiSum,
    overapproximate,
};
use nalgebra::{SMatrix, SVector};

const DIM: usize = 2;
type Float = f32;

fn main() {
    let rot = SMatrix::<Float, 2, 2>::from_vec(vec![
        Float::cos(0.4),
        -Float::sin(0.4),
        Float::sin(0.4),
        Float::cos(0.4),
    ]);

    let a_transform =
        SMatrix::<Float, 4, 2>::from_vec(vec![1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0]);
    let h = SVector::<Float, 4>::from_vec(vec![2.0, 2.0, 2.0, 2.0]);
    let polytope = SHalfspacePolytope::<Float, 4, 2>::new(a_transform, h);
    let lt = LinearTransformation::<Float, DIM>::new(rot, Box::new(polytope.clone()));

    // create minkowski sum
    let ms = MinkowskiSum::<Float, DIM>::new(Box::new(polytope.clone()), Box::new(lt));
    // sum with another ms
    let ms2 = MinkowskiSum::<Float, DIM>::new(Box::new(ms), Box::new(polytope.clone()));

    let oa = overapproximate::overapproximate::<Float, DIM>(&ms2);

    // print the oa a_tranform and upperbounds as csv
    for i in 0..oa.a_transform.nrows() {
        for j in 0..oa.a_transform.ncols() {
            print!("{},", oa.a_transform[(i, j)]);
        }
        print!("{}", oa.upper_bounds[i]);
        println!();
    }
}
