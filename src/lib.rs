use pyo3::prelude::*;
mod analyzer;
mod converters;
mod py_classes;

#[pymodule]
fn morphrs_py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<analyzer::PyMorphAnalyzer>()?;
    Ok(())
}
