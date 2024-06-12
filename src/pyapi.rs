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

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<u128> {
        slf.inner.next()
    }
}