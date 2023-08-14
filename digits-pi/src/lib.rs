use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Calculates an approximation of Pi using the Leibniz formula.
/// The more iterations, the more accurate the result.
#[pyfunction]
fn calculate_pi(iterations: u32) -> PyResult<f64> {
    let mut pi = 0.0;
    for k in 0..iterations {
        pi += ((-1.0f64).powi(k as i32) / (2 * k + 1) as f64) * 4.0;
    }
    Ok(pi)
}

/// A Python module implemented in Rust.
#[pymodule]
fn libdigits_pi(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_pi, m)?)?;
    Ok(())
}
