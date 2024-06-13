mod core;
pub mod feistel;
mod pyapi;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn smallperm(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<pyapi::PseudoRandomPermutation>()?;
    Ok(())
}
