use pyo3::types::PyString;
use pyo3::{prelude::*, types::PyModule};

fn marco_python(input: &str) -> PyResult<String> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let marco = PyModule::from_code(
            py,
            r#"
def marco(input):
    if input == "marco":
        return "python"
    else:
        return "no python"
"#,
            "marco.py",
            "marco",
        )?;
        let marco_func = marco.getattr("marco")?;
        let marco_result = marco_func.call1((input,))?;
        let marco_result: &PyString = marco_result.extract()?;
        Ok(marco_result.to_string())
    })
}

fn main() {
    println!("From embedded Python: {}", marco_python("marco").unwrap());
    println!("From embedded Python: {}", marco_python("polo").unwrap());
}
