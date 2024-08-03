use morph_rs::MorphAnalyzer;
use pyo3::prelude::*;
use std::path::Path;

#[pyclass]
#[pyo3(name = "MorphAnalyzer", subclass)]
struct PyMorphAnalyzer {
    morph_analyzer: MorphAnalyzer,
}

#[pymethods]
impl PyMorphAnalyzer {
    #[staticmethod]
    fn open(path: String) -> PyResult<Self> {
        let morph_analyzer = MorphAnalyzer::open(Path::new(&path)).unwrap();
        Ok(PyMorphAnalyzer { morph_analyzer })
    }

    fn parse_word(&self, word: String) -> PyResult<String> {
        Ok(self.morph_analyzer.parse_word(&word).unwrap().to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok("MorphAnalyzer()".to_string())
    }
}

#[pymodule]
fn morphrs_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyMorphAnalyzer>()?;
    Ok(())
}
