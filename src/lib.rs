use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn range_sum(a: usize, b: usize) -> PyResult<usize> {
    let mut number = a;
    let mut sum = 0;
    while number != b {
        sum += number;
        number += 1;
    }
    Ok(sum)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_cheatsheet(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(range_sum, m)?)?;

    Ok(())
}