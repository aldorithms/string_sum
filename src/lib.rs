use pyo3::prelude::*;

#[pyfunction]
fn sum(a: isize, b: isize) -> PyResult<isize> {
    Ok(a + b)
}

#[pyfunction]
fn difference(a: isize, b: isize) -> PyResult<isize> {
    Ok(a - b)
}

#[pyfunction]
fn product(a: isize, b: isize) -> PyResult<isize> {
    Ok(a * b)
}

#[pyfunction]
fn quotient(a: isize, b: isize) -> PyResult<isize> {
    Ok(a / b)
}

#[pyfunction]
fn remainder(a: isize, b: isize) -> PyResult<isize> {
    Ok(a % b)
}

#[pyfunction]
fn fibonacci(n: isize) -> PyResult<isize> {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    for _ in 0..n {
        c = a + b;
        a = b;
        b = c;
    }
    Ok(c)
}

#[pyfunction]
fn factorial(n: isize) -> PyResult<isize> {
    let mut result = 1;
    for i in 1..n+1 {
        result *= i;
    }
    Ok(result)
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: isize, b: isize) -> PyResult<String> {
    Ok(sum(a,b)?.to_string())
}

/// Formats the difference of two numbers as string.
#[pyfunction]
fn difference_as_string(a: isize, b: isize) -> PyResult<String> {
    Ok(difference(a, b)?.to_string())
}

/// Formats the product of two numbers as string.
#[pyfunction]
fn product_as_string(a: isize, b: isize) -> PyResult<String> {
    Ok(product(a,b)?.to_string())
}

/// Formats the quotient of two numbers as string.
#[pyfunction]
fn quotient_as_string(a: isize, b: isize) -> PyResult<String> {
    Ok(quotient(a, b)?.to_string())
}

/// Formats the remainder of two numbers as string.
#[pyfunction]
fn remainder_as_string(a: isize, b: isize) -> PyResult<String> {
    Ok(remainder(a, b)?.to_string())
}

#[pyfunction]
fn power_as_string(a: isize, b: isize) -> PyResult<String> {
    Ok((a.pow(b as u32)).to_string())
}

#[pyfunction]
fn fibonacci_as_string(n: isize) -> PyResult<String> {
    Ok(fibonacci(n)?.to_string())
}

#[pyfunction]
fn factorial_as_string(n: isize) -> PyResult<String> {
    Ok(factorial(n)?.to_string())
}

/// A Python module implemented in Rust. 
/// The name of this function must match the `lib.name` setting in the `Cargo.toml`,
/// else Python will not be able to import the module.
#[pymodule]
fn string_sum(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum, m)?)?;
    m.add_function(wrap_pyfunction!(difference, m)?)?;
    m.add_function(wrap_pyfunction!(product, m)?)?;
    m.add_function(wrap_pyfunction!(quotient, m)?)?;
    m.add_function(wrap_pyfunction!(remainder, m)?)?;
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(factorial, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(difference_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(product_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(quotient_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(remainder_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(power_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(fibonacci_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(factorial_as_string, m)?)?;
    Ok(())
}
