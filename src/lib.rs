use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
mod bert_tokenization;

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

fn range_sum_recursive_(a: usize, b: usize) -> usize{
    if a == b {
        return 0
    }else{
        return a + range_sum_recursive_(a + 1, b)
    }
}

#[pyfunction]
fn range_sum_recursive(a: usize, b: usize) -> PyResult<usize> {
    Ok(range_sum_recursive_(a, b))
}

#[pyfunction]
fn bert_tokenize(sent: &str) -> PyResult<String> {
    Ok(bert_tokenization::bert_tokenize(sent)?)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_cheatsheet(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(range_sum, m)?)?;
    m.add_function(wrap_pyfunction!(range_sum_recursive, m)?)?;

    m.add_function(wrap_pyfunction!(bert_tokenize, m)?)?;

    Ok(())
}