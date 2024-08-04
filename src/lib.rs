use morph_rs::{MorphAnalyzer, ParsedWords};
use pyo3::{prelude::*, types::PyList};
use std::path::Path;

#[pyclass]
#[pyo3(name = "MorphAnalyzer", subclass)]
struct PyMorphAnalyzer {
    morph_analyzer: MorphAnalyzer,
}

fn convert_parsed_word(parsed_word: morph_rs::ParsedWord) -> PyResult<PyParsedWord> {
    Ok(PyParsedWord {
        word: parsed_word.word(),
        tags: parsed_word
            .tag()
            .iter()
            .map(|tag| tag.to_string())
            .collect::<Vec<String>>(),
        normal_form: parsed_word.normal_form().to_string(),
        method: format!("{:?}", parsed_word.method()),
    })
}

fn convert_parsed_words(parsed_words: ParsedWords) -> PyResult<Py<PyList>> {
    Python::with_gil(|py| {
        let mut py_parsed_words = Vec::<Py<PyAny>>::new();
        for parsed_word in parsed_words.0 {
            let py_parsed_word = convert_parsed_word(parsed_word)?;
            py_parsed_words.push(py_parsed_word.into_py(py));
        }

        Ok(PyList::new(py, py_parsed_words.into_iter()).into())
    })
}

#[pyclass]
#[pyo3(name = "ParsedWord", subclass)]
struct PyParsedWord {
    #[pyo3(get)]
    word: String,

    #[pyo3(get)]
    tags: Vec<String>,

    #[pyo3(get)]
    normal_form: String,

    #[pyo3(get)]
    method: String,
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

#[pymodule]
fn morphrs_py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyMorphAnalyzer>()?;
    Ok(())
}
