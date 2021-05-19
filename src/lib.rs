use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
mod bert_tokenization;

/// add(a, b, /)
/// --
///
/// This function compute range sum between two integers.
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

#[pyfunction]
#[text_signature = "(vec1, vec2, /)"]
fn multiply(vec1: Vec<f32>, vec2: Vec<f32>) -> PyResult<Vec<f32>>{
    let mut ret: Vec<f32> = Vec::with_capacity(vec1.len());
    for i in 0..vec1.len(){
        ret.push(vec1[i] * vec2[i]);
    }
    return Ok(ret);
}

#[pyfunction]
fn merge_count(
    count1: HashMap<String, i32>, 
    count2: HashMap<String, i32>
) -> PyResult<HashMap<String, i32>>{
    let mut ret = count1.clone();
    for (k, v) in count2.into_iter(){
        if !ret.contains_key(&k){
            ret.insert(k, v);
        }else{
            let to_add = *(ret.get(&k).unwrap());
            ret.insert(k, v + to_add);
        }
    }
    return Ok(ret);
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_cheatsheet(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(range_sum, m)?)?;
    m.add_function(wrap_pyfunction!(range_sum_recursive, m)?)?;

    m.add_function(wrap_pyfunction!(bert_tokenize, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(merge_count, m)?)?;


    Ok(())
}