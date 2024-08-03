use morph_rs::MorphAnalyzer;
use pyo3::{prelude::*, types::{PyString, PyType}};
use std::path::Path;

#[pyclass]
#[pyo3(name = "MorphAnalyzer", subclass)]
struct PyMorphAnalyzer {
    // path field available for getting and setting from python code
    #[pyo3(get, set)]
    path: String,
    // morph_analyzer: MorphAnalyzer
}

#[pymethods]
impl PyMorphAnalyzer {
    #[new]
    fn new(path: String) -> Self {
        PyMorphAnalyzer {
            path
        }
    }
    
    #[staticmethod]
    fn open(path: String) -> PyResult<Self> {
        // let morph_analyzer = MorphAnalyzer::open(Path::new(path.extract().unwrap()));

        // let py_morph_analyzer = cls.call((), None)?;
        let result = PyMorphAnalyzer {
            path
        };
        Ok(result)
    }
    
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("MorphAnalyzer(\"{}\")", self.path))
    }
}

#[pymodule]
fn morphrs_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyMorphAnalyzer>()?;
    Ok(())
}
