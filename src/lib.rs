use std::collections::HashMap;

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn fibonacci(number: i64) -> PyResult<usize> {
    if number < 0 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "n must be >= 0",
        ));
    }
    match number {
        0 | 1 | 2 => Ok(1),
        _ => Ok(fibonacci(number - 1)? + fibonacci(number - 2)?),
    }
}

#[pyfunction]
fn fibonacci_number_map(numbers: Vec<i64>) -> Py<PyAny> {
    let mut n_map = HashMap::new();
    for &n in numbers.iter() {
        let value = n_map.entry(n.to_string()).or_insert(0);
        *value = fibonacci(n)
            .map_err(|e| {
                Python::with_gil(|py| {
                    e.restore(py);
                })
            })
            .unwrap_or(0);
    }

    return Python::with_gil(|py| n_map.to_object(py));
}

/// A Python module implemented in Rust.
#[pymodule]
fn rutils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(fibonacci_number_map, m)?)?;
    Ok(())
}
