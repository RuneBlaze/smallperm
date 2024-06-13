use pyo3::prelude::*;

use crate::feistel::Permutor;

#[pyclass]
pub struct PseudoRandomPermutation {
    inner: Permutor,
}

#[pymethods]
impl PseudoRandomPermutation {
    #[new]
    fn new(max: u128, key: u64) -> Self {
        let inner = Permutor::new_with_u64_key(max, key);
        PseudoRandomPermutation { inner }
    }

    fn __iter__(&self) -> Self {
        let mut new_inner = self.inner.clone();
        new_inner.values_returned = 0;
        PseudoRandomPermutation { inner: new_inner }
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<u128> {
        slf.inner.next()
    }

    fn __len__(&self) -> usize {
        self.inner.max as usize
    }

    fn __getitem__(&self, index: isize) -> PyResult<u128> {
        if index < 0 {
            return Err(pyo3::exceptions::PyIndexError::new_err("negative index"));
        }
        if index as u128 >= self.inner.max {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "index out of range",
            ));
        }
        Ok(self.inner.forward(index as u128))
    }

    fn forward(&self, ix: u128) -> u128 {
        if ix >= self.inner.max {
            panic!("index out of range");
        }
        self.inner.forward(ix)
    }

    fn backward(&self, permuted_ix: u128) -> u128 {
        if permuted_ix >= self.inner.max {
            panic!("index out of range");
        }
        self.inner.backward(permuted_ix)
    }
}
