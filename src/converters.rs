use morph_rs::{
    morph::grammemes::{
        Animacy, Aspect, Case, Gender, Grammem, Involvement, Mood, Number, Other, ParteSpeech,
        Tense, Transitivity, Voice,
    },
    ParsedWord, ParsedWords,
};

use pyo3::prelude::*;

use crate::py_classes::PyParsedWord;

fn convert_grammem(grammem: &Grammem) -> String {
    grammem.to_string()
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

pub fn convert_parsed_words(parsed_words: ParsedWords) -> PyResult<Vec<PyParsedWord>> {
    let mut py_parsed_words = Vec::<PyParsedWord>::new();
    for parsed_word in parsed_words.0 {
        let py_parsed_word = convert_parsed_word(parsed_word)?;
        py_parsed_words.push(py_parsed_word);
    }

    Ok(py_parsed_words)
}

fn convert_to_grammem(grammem: &str) -> Option<Grammem> {
    match grammem {
        // Parts of speech
        "Noun" => Some(Grammem::ParteSpeech(ParteSpeech::Noun)),
        "AdjectiveFull" => Some(Grammem::ParteSpeech(ParteSpeech::AdjectiveFull)),
        "AdjectiveShort" => Some(Grammem::ParteSpeech(ParteSpeech::AdjectiveShort)),
        "Comparative" => Some(Grammem::ParteSpeech(ParteSpeech::Comparative)),
        "Verb" => Some(Grammem::ParteSpeech(ParteSpeech::Verb)),
        "Infinitive" => Some(Grammem::ParteSpeech(ParteSpeech::Infinitive)),
        "ParticipleFull" => Some(Grammem::ParteSpeech(ParteSpeech::ParticipleFull)),
        "ParticipleShort" => Some(Grammem::ParteSpeech(ParteSpeech::ParticipleShort)),
        "Gerundive" => Some(Grammem::ParteSpeech(ParteSpeech::Gerundive)),
        "Number" => Some(Grammem::ParteSpeech(ParteSpeech::Number)),
        "Adverb" => Some(Grammem::ParteSpeech(ParteSpeech::Adverb)),
        "NounPronoun" => Some(Grammem::ParteSpeech(ParteSpeech::NounPronoun)),
        "Predicative" => Some(Grammem::ParteSpeech(ParteSpeech::Predicative)),
        "Conjunction" => Some(Grammem::ParteSpeech(ParteSpeech::Conjunction)),
        "Particle" => Some(Grammem::ParteSpeech(ParteSpeech::Particle)),
        "Interjection" => Some(Grammem::ParteSpeech(ParteSpeech::Interjection)),

        // Animacy
        "Animate" => Some(Grammem::Animacy(Animacy::Animate)),
        "Inanimate" => Some(Grammem::Animacy(Animacy::Inanimate)),

        // Aspect
        "Perfetto" => Some(Grammem::Aspect(Aspect::Perfetto)),
        "Imperfetto" => Some(Grammem::Aspect(Aspect::Imperfetto)),

        // Number
        "Singular" => Some(Grammem::Number(Number::Singular)),
        "Plural" => Some(Grammem::Number(Number::Plural)),
        "SingulariaTantum" => Some(Grammem::Number(Number::SingulariaTantum)),
        "PluraliaTantum" => Some(Grammem::Number(Number::PluraliaTantum)),

        // Transitivity
        "Transitive" => Some(Grammem::Trans(Transitivity::Transitive)),
        "Intransitive" => Some(Grammem::Trans(Transitivity::Intransitive)),

        // Tense
        "Past" => Some(Grammem::Tense(Tense::Past)),
        "Present" => Some(Grammem::Tense(Tense::Present)),
        "Future" => Some(Grammem::Tense(Tense::Future)),

        // Case
        "Fixed" => Some(Grammem::Case(Case::Fixed)),
        "Nominativus" => Some(Grammem::Case(Case::Nominativus)),
        "Genitivus" => Some(Grammem::Case(Case::Genetivus)),
        "Dativus" => Some(Grammem::Case(Case::Dativus)),
        "Accusativus" => Some(Grammem::Case(Case::Accusativus)),
        "Ablativus" => Some(Grammem::Case(Case::Ablativus)),
        "Locativus" => Some(Grammem::Case(Case::Locativus)),
        "Vocativus" => Some(Grammem::Case(Case::Vocativus)),
        "Gen2" => Some(Grammem::Case(Case::Gen2)),
        "Acc2" => Some(Grammem::Case(Case::Acc2)),
        "Loc2" => Some(Grammem::Case(Case::Loc2)),

        // Gender
        "Masculine" => Some(Grammem::Gender(Gender::Masculine)),
        "Feminine" => Some(Grammem::Gender(Gender::Feminine)),
        "Neutral" => Some(Grammem::Gender(Gender::Neutral)),
        "Common" => Some(Grammem::Gender(Gender::Common)),
        "CommonWavering" => Some(Grammem::Gender(Gender::CommonWavering)),
        "GenderNeutral" => Some(Grammem::Gender(Gender::GenderNeutral)),

        // Mood
        "Indicativo" => Some(Grammem::Mood(Mood::Indicativo)),
        "Imperativo" => Some(Grammem::Mood(Mood::Imperativo)),

        // Voice
        "Active" => Some(Grammem::Voice(Voice::Active)),
        "Passive" => Some(Grammem::Voice(Voice::Passive)),

        // Involvement
        "Incluso" => Some(Grammem::Involvement(Involvement::Incluso)),
        "Excluso" => Some(Grammem::Involvement(Involvement::Excluso)),

        // Other
        "Abbreviation" => Some(Grammem::Other(Other::Abbreviation)),
        "Name" => Some(Grammem::Other(Other::Name)),
        "Surname" => Some(Grammem::Other(Other::Surname)),
        "Patronymic" => Some(Grammem::Other(Other::Patronymic)),
        "Geography" => Some(Grammem::Other(Other::Geography)),
        "Organization" => Some(Grammem::Other(Other::Organization)),
        "Trademark" => Some(Grammem::Other(Other::Trademark)),
        "PossibleSubstantive" => Some(Grammem::Other(Other::PossibleSubstantive)),
        "Superior" => Some(Grammem::Other(Other::Superior)),
        "Ordinal" => Some(Grammem::Other(Other::Ordinal)),
        "Possessive" => Some(Grammem::Other(Other::Possessive)),
        "Questionable" => Some(Grammem::Other(Other::Questionable)),
        "Demonstrative" => Some(Grammem::Other(Other::Demonstrative)),
        "Anaphoric" => Some(Grammem::Other(Other::Anaphoric)),

        "Comparative" => Some(Grammem::Other(Other::Comparative)),
        "FormEY" => Some(Grammem::Other(Other::FormEY)),
        "FormOY" => Some(Grammem::Other(Other::FormOY)),
        "FormEJ" => Some(Grammem::Other(Other::FormEJ)),
        "FormBE" => Some(Grammem::Other(Other::FormBE)),
        "FormENEN" => Some(Grammem::Other(Other::FormENEN)),
        "FormIE" => Some(Grammem::Other(Other::FormIE)),
        "FormBI" => Some(Grammem::Other(Other::FormBI)),
        "ParticipleSH" => Some(Grammem::Other(Other::ParticipleSH)),

        "Multiple" => Some(Grammem::Other(Other::Multiple)),
        "Reflessivo" => Some(Grammem::Other(Other::Reflessivo)),
        "Spoken" => Some(Grammem::Other(Other::Spoken)),
        "Slang" => Some(Grammem::Other(Other::Slang)),
        "Archaic" => Some(Grammem::Other(Other::Archaic)),
        "Literary" => Some(Grammem::Other(Other::Literary)),
        "Error" => Some(Grammem::Other(Other::Error)),
        "Distortion" => Some(Grammem::Other(Other::Distortion)),
        "Parenthesis" => Some(Grammem::Other(Other::Parenthesis)),
        "ImperfectiveParticiple" => Some(Grammem::Other(Other::ImperfectiveParticiple)),
        "PossiblePredicative" => Some(Grammem::Other(Other::PossiblePredicative)),
        "Countable" => Some(Grammem::Other(Other::Countable)),
        "Collection" => Some(Grammem::Other(Other::Collection)),
        "AfterPreposition" => Some(Grammem::Other(Other::AfterPreposition)),
        "PrepositionVariant" => Some(Grammem::Other(Other::PrepositionVariant)),
        "Initial" => Some(Grammem::Other(Other::Initial)),
        "PossibleAdjective" => Some(Grammem::Other(Other::PossibleAdjective)),
        "Hypothetical" => Some(Grammem::Other(Other::Hypothetical)),
        "Other" => Some(Grammem::Other(Other::Other)),

        _ => None,
    }
}
