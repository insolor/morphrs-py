use pyo3::prelude::*;

#[pyclass]
#[pyo3(name = "MorphAnalyzer", subclass)]
struct PyMorphAnalyzer {
    num: i32,
}

#[pymodule]
fn morphrs_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyMorphAnalyzer>()?;
    Ok(())
}
