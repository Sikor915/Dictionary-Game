use anyhow::{bail, Result};
use json::parse;

use super::Dictionary;
use std::ops::Index;

pub struct WordDefinitionPair {
    word: String,
    definition: String
}

pub struct JsonDictionary {
    data: Vec<WordDefinitionPair>
}

impl JsonDictionary {
    pub fn new(json: &str) -> Result<JsonDictionary> {
        let parsed = json::parse(json)?;
        let mut data: Vec<WordDefinitionPair> = Vec::new();

        if parsed.is_array() == false {
            bail!("Json object is not an array!");
        }

        for i in 0..parsed.len() {
            let element = &parsed[i];
            let wordOpt = element["WORD"].as_str();
            let definitionOpt = element["DEFINITION"].as_str();

            let pair = match(wordOpt, definitionOpt) {
                (Some(word), Some(definition)) => {
                    WordDefinitionPair {
                        word: word.to_string(),
                        definition: definition.to_string()
                    }
                },
                _ => {
                    bail!("Word or definition not found");
                }
            };

            data.push(pair);
        }

        Ok(Self {
            data
        })
    }
}

impl Index<usize> for JsonDictionary {
    type Output = WordDefinitionPair;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Dictionary for JsonDictionary {
    fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::dictionary;

    use super::*;

    #[test]
    fn return_len_0_on_empty_json() {
        let json = "[]";
        let dictionary = JsonDictionary::new(json).unwrap();

        assert_eq!(dictionary.len(), 0);
    }

    #[test]
    fn return_error_on_root_object_instead_of_array() {
        let json = "{}";
        let dictionary = JsonDictionary::new(json);

        assert_eq!(dictionary.is_err(), true);
    }

    #[test]
    fn return_error_on_invalid_json() {
        let json = "{invalid}";
        let dictionary = JsonDictionary::new(json);

        assert_eq!(dictionary.is_err(), true);
    }

    #[test]
    fn return_len_1_on_valid_json_with_1_pair() {
        let json = r#"
        [
            {
                "WORD": "Rust",
                "DEFINITION": "Awesome Language!"
            }
        ]
        "#;
        let dictionary = JsonDictionary::new(json).unwrap();

        assert_eq!(dictionary.len(), 1);
        assert_eq!(dictionary[0].word, "Rust");
        assert_eq!(dictionary[0].definition, "Awesome Language!");
    }
}