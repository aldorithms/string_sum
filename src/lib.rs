use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: isize, b: isize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Formats the difference of two numbers as string.
#[pyfunction]
fn difference_as_string(a: isize, b: isize) -> PyResult<String> {
    Ok((a - b).to_string())
}

/// Formats the product of two numbers as string.
#[pyfunction]
fn product_as_string(a: isize, b: isize) -> PyResult<String> {
    Ok((a * b).to_string())
}

/// Formats the quotient of two numbers as string.
#[pyfunction]
fn quotient_as_string(a: isize, b: isize) -> PyResult<String> {
    Ok((a / b).to_string())
}

/// Formats the remainder of two numbers as string.
#[pyfunction]
fn remainder_as_string(a: isize, b: isize) -> PyResult<String> {
    Ok((a % b).to_string())
}

/// A Python module implemented in Rust. 
/// The name of this function must match the `lib.name` setting in the `Cargo.toml`,
/// else Python will not be able to import the module.
#[pymodule]
fn string_sum(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(difference_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(product_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(quotient_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(remainder_as_string, m)?)?;
    Ok(())
}
