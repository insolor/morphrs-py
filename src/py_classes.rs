use pyo3::prelude::*;

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
