use lazy_reach::{
    convex::SHalfspacePolytope, lazy_operation::ConvexHull, lazy_operation::LinearTransformation,
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
    let h = SVector::<Float, 4>::from_vec(vec![1.0, 1.0, 1.0, 1.0]);
    let polytope = SHalfspacePolytope::<Float, 4, 2>::new(a_transform, h);
    let lt = LinearTransformation::<Float, DIM>::new(rot, Box::new(polytope.clone()));

    let a_transform2 =
        SMatrix::<Float, 4, 2>::from_vec(vec![1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0]);
    let h2 = SVector::<Float, 4>::from_vec(vec![4.0, -2.0, 4.0, -2.0]);
    let polytope1 = SHalfspacePolytope::<Float, 4, 2>::new(a_transform2, h2);

    let ch = ConvexHull::<Float, DIM>::new(Box::new(lt), Box::new(polytope1));

    let oa = overapproximate::overapproximate::<Float, DIM>(&ch, 20);

    // print the oa a_tranform and upperbounds as csv
    for i in 0..oa.a_transform.nrows() {
        for j in 0..oa.a_transform.ncols() {
            print!("{},", oa.a_transform[(i, j)]);
        }
        print!("{}", oa.upper_bounds[i]);
        println!();
    }
}
