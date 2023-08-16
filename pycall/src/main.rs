use pyo3::prelude::*;

fn main() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    let values = vec![1, 2, 3];

    println!("Passing values to Python to sum: {:?}\n", values);
    Python::with_gil(|py| {
        let builtins = PyModule::import(py, "builtins")?;
        let total: i32 = builtins.getattr("sum")?.call1((values,))?.extract()?;
        println!("total sum from Python: {}\n", total);

        let os = PyModule::import(py, "os")?;
        let user: String = os.getattr("getenv")?.call1(("USER",))?.extract()?;
        println!("User from Python: {}\n", user);
        Ok(())
    })
}
