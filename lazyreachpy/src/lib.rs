use lazy_reach::convex::{DHalfspacePolytope, DSupportFunction, SupportFunction};
use nalgebra::{Const, Dynamic, VecStorage};
use nalgebra::{DMatrix, DVector};
use numpy::convert::ToPyArray;
use numpy::{PyArray1, PyReadonlyArrayDyn};
use pyo3::prelude::*;

// TODO: do better here
fn numpy2dmatrix(a: PyReadonlyArrayDyn<f64>) -> DMatrix<f64> {
    let a = a.as_array();
    let shape = a.shape();
    let mut H = Vec::new();
    for i in 0..shape[0] {
        for j in 0..shape[1] {
            H.push(a[[i, j]]);
        }
    }
    DMatrix::<f64>::from_vec(shape[1], shape[0], H).transpose()
}

fn numpy2dvector(a: PyReadonlyArrayDyn<f64>) -> DVector<f64> {
    // TODO: clean this up
    let a = a.as_array();
    let shape = a.shape();
    if shape.len() == 2 {
        let mut H = Vec::new();
        for i in 0..shape[0] {
            H.push(a[[i, 0]]);
        }
        DVector::<f64>::from_vec(H)
    } else {
        let mut H = Vec::new();
        for i in 0..shape[0] {
            H.push(a[[i]]);
        }
        DVector::<f64>::from_vec(H)
    }
}

#[pyclass(subclass)]
pub struct HPolytope {
    inner: DHalfspacePolytope<f64>,
}

#[pymethods]
impl HPolytope {
    /// new pass numpy array
    #[new]
    fn new(A: PyReadonlyArrayDyn<f64>, b: PyReadonlyArrayDyn<f64>) -> HPolytope {
        let A = numpy2dmatrix(A);
        let b = numpy2dvector(b);

        HPolytope {
            inner: DHalfspacePolytope::<f64>::new(A, b),
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "HPolytope: A: {:?}, b: {:?}",
            self.inner.a_transform.shape(),
            self.inner.upper_bounds.shape()
        ))
    }

    fn support_function<'py>(
        &self,
        direction: PyReadonlyArrayDyn<f64>,
        py: Python<'py>,
    ) -> (f64, &'py PyArray1<f64>) {
        let d = numpy2dvector(direction);
        let (value, vector) = self.inner.support(&d);
        (value, vector.data.as_vec().to_pyarray(py))
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn lazyreachpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<HPolytope>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
