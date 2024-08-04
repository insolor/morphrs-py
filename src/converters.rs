use morph_rs::{morph::grammemes::Grammem, InflectWord, ParsedWord};

use pyo3::prelude::*;

use crate::py_classes::{PyInflectWord, PyParsedWord};

fn convert_grammem(grammem: &Grammem) -> String {
    let grammem = serde_json::to_string(&grammem).unwrap();
    grammem[1..grammem.len() - 1].to_string()
}

fn convert_tag(tag: Vec<Grammem>) -> Vec<String> {
    tag.iter().map(convert_grammem).collect::<Vec<String>>()
}

fn convert_parsed_word(parsed_word: ParsedWord) -> PyResult<PyParsedWord> {
    Ok(PyParsedWord {
        word: parsed_word.word(),
        tags: convert_tag(parsed_word.tag().into_vec()),
        normal_form: parsed_word.normal_form().to_string(),
        method: format!("{:?}", parsed_word.method()),
    })
}

pub fn convert_parsed_words(parsed_words: Vec<ParsedWord>) -> PyResult<Vec<PyParsedWord>> {
    let mut py_parsed_words = Vec::<PyParsedWord>::new();
    for parsed_word in parsed_words {
        let py_parsed_word = convert_parsed_word(parsed_word)?;
        py_parsed_words.push(py_parsed_word);
    }

    Ok(py_parsed_words)
}

fn convert_to_grammem(grammem: &str) -> Grammem {
    let quoted = format!("\"{}\"", grammem);
    let deserialized: Grammem = serde_json::from_str(&quoted).unwrap();
    deserialized
}

pub fn convert_to_convert_to_grammems(grammems: Vec<String>) -> Vec<Grammem> {
    grammems.iter().map(|gram| convert_to_grammem(&gram)).collect::<Vec<Grammem>>()
}

pub fn convert_inflected_word(inflected_word: &InflectWord) -> PyInflectWord {
    PyInflectWord {
        word: inflected_word.word(),
        tags: convert_tag(inflected_word.tag().into_vec()),
        method: format!("{:?}", inflected_word.method()),
    }
}

#[cfg(test)]
mod tests {
    use morph_rs::morph::grammemes::Animacy;

    use super::*;
    
    #[test]
    fn test_convert_to_grammem() {
        let source_grammem = Grammem::Animacy(Animacy::Inanimate);
        let grammem_string: String = convert_grammem(&source_grammem);
        assert_eq!(convert_to_grammem(&grammem_string), source_grammem);
    }
}
