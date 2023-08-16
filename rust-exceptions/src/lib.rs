use pyo3::prelude::*;
use pyo3::exceptions::PyZeroDivisionError;

/// Divides a by b
/// Raises ZeroDivisionError if b is zero
#[pyfunction]
fn divide(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        return Err(PyZeroDivisionError::new_err("Exception:  !Division by Zero"));
    }
    Ok(a / b)
}

/// A Python module implemented in Rust.
#[pymodule]
fn librust_exceptions(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(divide, m)?)?;
    Ok(())
}