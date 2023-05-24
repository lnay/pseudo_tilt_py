use pyo3::prelude::*;
use pseudo_tilt::chern_character::{ChernChar, ChowGens, Δ};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn bogomolov_form(r: i32, c: i32, d: i32) -> PyResult<i32> {
    const P1: ChowGens = ChowGens{a: 1, b: 1, c: 1};
    let v = ChernChar::<P1>{r, c, d};
    Ok(Δ(&v))
}

/// A Python module implemented in Rust.
#[pymodule]
fn tilt_rs_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(bogomolov_form, m)?)?;
    Ok(())
}
