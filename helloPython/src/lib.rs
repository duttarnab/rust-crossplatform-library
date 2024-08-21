use pyo3::prelude::*;
use hello::hello;

#[pyfunction]
fn string_from_core() -> PyResult<String> {
    let str = String::from(hello::greetings_from_rust());
    println!("{}", str);
    Ok(str.to_string())
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn helloPython(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(string_from_core, m)?)?;
    Ok(())
}

