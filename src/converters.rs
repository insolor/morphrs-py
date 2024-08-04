use morph_rs::{ParsedWord, ParsedWords};
use pyo3::prelude::*;

use crate::py_classes::PyParsedWord;

fn convert_parsed_word(parsed_word: ParsedWord) -> PyResult<PyParsedWord> {
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

pub fn convert_parsed_words(parsed_words: ParsedWords) -> PyResult<Vec<PyParsedWord>> {
    let mut py_parsed_words = Vec::<PyParsedWord>::new();
    for parsed_word in parsed_words.0 {
        let py_parsed_word = convert_parsed_word(parsed_word)?;
        py_parsed_words.push(py_parsed_word);
    }

    Ok(py_parsed_words)
}
