use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;

/// Demonstrates conversions between Rust and Python data types.
#[pyfunction]
fn data_types_example(py: Python<'_>) -> PyResult<PyObject> {
    let text: &str = "Hello, Python!";
    let integer: i32 = 42;
    let floating: f64 = 3.14;
    let boolean: bool = true;

    // Create a Python dictionary
    let python_dict = PyDict::new(py);

    // Insert key-value pairs, converting Rust types to Python objects
    python_dict.set_item("text", text)?;
    python_dict.set_item("integer", integer)?;
    python_dict.set_item("floating", floating)?;
    python_dict.set_item("boolean", boolean)?;

    // Return the Python dictionary
    Ok(python_dict.to_object(py))
}


/// A Python module implemented in Rust.
#[pymodule]
fn libdata_conversion(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(data_types_example, m)?)?;
    Ok(())
}