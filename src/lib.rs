use pyo3::prelude::*;
mod converters;
mod py_classes;

#[pymodule]
fn morphrs_py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<py_classes::PyMorphAnalyzer>()?;
    Ok(())
}
