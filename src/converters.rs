use morph_rs::ParsedWords;
use pyo3::{prelude::*, types::PyList};

use crate::py_classes::PyParsedWord;

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

pub fn convert_parsed_words(parsed_words: ParsedWords) -> PyResult<Py<PyList>> {
    Python::with_gil(|py| {
        let mut py_parsed_words = Vec::<Py<PyAny>>::new();
        for parsed_word in parsed_words.0 {
            let py_parsed_word = convert_parsed_word(parsed_word)?;
            py_parsed_words.push(py_parsed_word.into_py(py));
        }

        Ok(PyList::new(py, py_parsed_words.into_iter()).into())
    })
}
