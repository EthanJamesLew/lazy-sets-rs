/*!
 * Convert between numpy rust and other array types
 */

use std::ptr;

use nalgebra::{storage::Storage, DMatrix, DVector};
use numpy::{PyArray1, PyArray2, PyReadonlyArrayDyn};
use pyo3::Python;

/// to 2D numpy array
pub trait ToPyArray2 {
    fn to_pyarray<'py>(&self, py: Python<'py>) -> &'py PyArray2<f64>;
}

/// to 1D numpy array
pub trait ToPyArray1 {
    fn to_pyarray<'py>(&self, py: Python<'py>) -> &'py PyArray1<f64>;
}

/// to nalgebra DMatrix
pub trait ToDMatrix {
    fn to_dmatrix(&self) -> DMatrix<f64>;
}

/// to nalgebra DVector
pub trait ToDVector {
    fn to_dvector(&self) -> DVector<f64>;
}

// TODO: make this generic over floats?
impl ToPyArray2 for DMatrix<f64> {
    fn to_pyarray<'py>(&self, py: Python<'py>) -> &'py PyArray2<f64> {
        unsafe {
            let array = PyArray2::new(py, (self.nrows(), self.ncols()), true);
            let mut data_ptr = array.data();
            if self.data.is_contiguous() {
                ptr::copy_nonoverlapping(self.data.ptr(), data_ptr, self.len());
            } else {
                for item in self.iter() {
                    data_ptr.write(item.clone());
                    data_ptr = data_ptr.add(1);
                }
            }
            array
        }
    }
}

impl ToPyArray1 for DVector<f64> {
    fn to_pyarray<'py>(&self, py: Python<'py>) -> &'py PyArray1<f64> {
        unsafe {
            let array = PyArray1::new(py, self.len(), true);
            let mut data_ptr = array.data();
            if self.data.is_contiguous() {
                ptr::copy_nonoverlapping(self.data.ptr(), data_ptr, self.len());
            } else {
                for item in self.iter() {
                    data_ptr.write(item.clone());
                    data_ptr = data_ptr.add(1);
                }
            }
            array
        }
    }
}

impl ToDMatrix for PyReadonlyArrayDyn<'_, f64> {
    fn to_dmatrix(&self) -> DMatrix<f64> {
        let a = self.as_array();
        let shape = a.shape();
        let mut h_mat = Vec::new();
        for i in 0..shape[0] {
            for j in 0..shape[1] {
                h_mat.push(a[[i, j]]);
            }
        }
        DMatrix::from_row_slice(shape[0], shape[1], &h_mat)
    }
}

impl ToDVector for PyReadonlyArrayDyn<'_, f64> {
    fn to_dvector(&self) -> DVector<f64> {
        let a = self.as_array();
        let shape = a.shape();
        let mut h_vec = Vec::new();
        for i in 0..shape[0] {
            h_vec.push(a[[i]]);
        }
        DVector::from_vec(h_vec)
    }
}
