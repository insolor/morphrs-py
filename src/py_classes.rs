use morph_rs::MorphAnalyzer;
use pyo3::prelude::*;
use std::path::Path;

use crate::converters::{convert_inflected_word, convert_parsed_words, convert_to_convert_to_grammems};

#[pyclass]
#[pyo3(name = "MorphAnalyzer", subclass)]
pub struct PyMorphAnalyzer {
    morph_analyzer: MorphAnalyzer,
}

#[pymethods]
impl PyMorphAnalyzer {
    #[staticmethod]
    fn open(path: &str) -> PyResult<Self> {
        let morph_analyzer = MorphAnalyzer::open(Path::new(path)).unwrap();
        Ok(PyMorphAnalyzer { morph_analyzer })
    }

    fn parse_word(&self, word: &str) -> PyResult<Vec<PyParsedWord>> {
        let parsed_words = self.morph_analyzer.parse_word(word).unwrap();
        Ok(convert_parsed_words(parsed_words.0).unwrap())
    }

    fn inflect_forms(&self, word: &str, tags: Vec<String>) -> PyResult<Vec<PyInflectWord>> {
        let rs_grammemes = convert_to_convert_to_grammems(tags);
        let inflected_words = self.morph_analyzer.inflect_forms(word, rs_grammemes).unwrap();
        match inflected_words {
            Some(inflected_words) => {
                Ok(inflected_words.0.iter().map(convert_inflected_word).collect())
            },
            None => Ok(Vec::new()),
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok("MorphAnalyzer()".to_string())
    }
}

#[pyclass]
#[pyo3(name = "ParsedWord", subclass)]
pub struct PyParsedWord {
    #[pyo3(get)]
    pub word: String,

    #[pyo3(get)]
    pub tags: Vec<String>,

    #[pyo3(get)]
    pub normal_form: String,

    #[pyo3(get)]
    pub method: String,
}

#[pymethods]
impl PyParsedWord {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "ParsedWord(word=\"{}\", tags=[{}], normal_form=\"{}\", method={:?})",
            self.word,
            self.tags.join(", "),
            self.normal_form,
            self.method
        ))
    }
}

#[pyclass]
#[pyo3(name = "InflectWord", subclass)]
pub struct PyInflectWord {
    #[pyo3(get)]
    pub word: String,

    #[pyo3(get)]
    pub tags: Vec<String>,

    #[pyo3(get)]
    pub method: String,
}

#[pymethods]
impl PyInflectWord {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "InflectWord(word=\"{}\", tags=[{}], method={:?})",
            self.word,
            self.tags.join(", "),
            self.method
        ))
    }
}