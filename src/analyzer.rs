use morph_rs::MorphAnalyzer;
use pyo3::{prelude::*, types::PyList};
use std::path::Path;

use crate::converters::convert_parsed_words;

#[pyclass]
#[pyo3(name = "MorphAnalyzer", subclass)]
pub struct PyMorphAnalyzer {
    morph_analyzer: MorphAnalyzer,
}

#[pymethods]
impl PyMorphAnalyzer {
    #[staticmethod]
    fn open(path: String) -> PyResult<Self> {
        let morph_analyzer = MorphAnalyzer::open(Path::new(&path)).unwrap();
        Ok(PyMorphAnalyzer { morph_analyzer })
    }

    fn parse_word(&self, word: String) -> Py<PyList> {
        let parsed_words = self.morph_analyzer.parse_word(&word).unwrap();
        convert_parsed_words(parsed_words).unwrap()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok("MorphAnalyzer()".to_string())
    }
}
