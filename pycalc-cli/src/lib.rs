/*
Calculator functions to later import into a Python Fire CLI.
*/

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Formats adding two numbers as string.
#[pyfunction]
fn add_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Formats subtracting two numbers as string.
#[pyfunction]
fn subtract_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a - b).to_string())
}

/// Formats dividing two numbers as string.
#[pyfunction]
fn divide_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a / b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn libpycalc_cli(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(add_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(subtract_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(divide_as_string, m)?)?;
    Ok(())
}