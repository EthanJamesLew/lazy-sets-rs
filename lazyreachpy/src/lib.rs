pub mod numpyffi;
use crate::numpyffi::convert::{ToDMatrix, ToDVector, ToPyArray1};

use lazy_reach::convex::{DHalfspacePolytope, DSupportFunction};
use numpy::{PyArray1, PyReadonlyArrayDyn};
use pyo3::prelude::*;

#[pyclass(subclass)]
pub struct HPolytope {
    inner: DHalfspacePolytope<f64>,
}

#[pymethods]
impl HPolytope {
    /// new pass numpy array
    #[new]
    fn new(a_transform: PyReadonlyArrayDyn<f64>, b: PyReadonlyArrayDyn<f64>) -> HPolytope {
        let a_transform = a_transform.to_dmatrix();
        let b = b.to_dvector();

        HPolytope {
            inner: DHalfspacePolytope::<f64>::new(a_transform, b),
        }
    }

    /// string representation
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "HPolytope: A: {:?}, b: {:?}",
            self.inner.a_transform.shape(),
            self.inner.upper_bounds.shape()
        ))
    }

    /// call support function
    fn support_function<'py>(
        &self,
        direction: PyReadonlyArrayDyn<f64>,
        py: Python<'py>,
    ) -> (f64, &'py PyArray1<f64>) {
        let d = direction.to_dvector();
        let (value, vector) = self.inner.support(&d);
        (value, vector.to_pyarray(py))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn lazyreachpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<HPolytope>()?;
    Ok(())
}
