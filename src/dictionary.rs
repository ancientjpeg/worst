use crate::error::Error;
use rq;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct WiktionaryDefinition {
    definition: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)] // I don't have a Damn Choice...
struct WiktionaryUsage {
    partOfSpeech: String,
    language: String,
    definitions: Vec<WiktionaryDefinition>,
}

#[derive(Debug, Deserialize, Serialize)]
struct WiktionaryResponse {
    en: Vec<WiktionaryUsage>,
}

pub struct Defintion {
    pub word: String,
    pub definition: String,
}

fn parse_html(mut text: String) -> Result<String, Error> {
    while let Some(idx) = text.find('<') {
        let end_idx = text.find('>').map_or(text.len(), |i| i + 1);

        if end_idx == text.len() && text.chars().rev().next() != Some('>') {
            return Err(Error::from("Error while removing tags from html text"));
        }

        text.replace_range(idx..end_idx, "");
    }

    Ok(text)
}

fn map_err_to_str<T>(e: T) -> String
where
    T: ToString,
{
    e.to_string()
}

pub fn get_rq(word: &str) -> Result<Defintion, Error> {
    let url = format!(
        "https://en.wiktionary.org/api/rest_v1/page/definition/{}",
        word
    );

    let full_url =
        rq::Url::parse_with_params(&url, &[("redirect", "false")]).map_err(map_err_to_str)?;

    let result = rq::blocking::get(full_url).map_err(map_err_to_str)?;

    let text = result.text().map_err(map_err_to_str)?;

    let wiki_res = serde_json::from_str::<WiktionaryResponse>(&text);

    let wiki = match wiki_res {
        Ok(w) => w,
        Err(e) => return Err(Error::from(e)),
    };

    let d = Defintion {
        word: String::from(word),
        definition: parse_html(String::from(&wiki.en[0].definitions[0].definition))?,
    };

    Ok(d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_parse() {
        let s = "<tag>text</tag>";
        let comp = Ok("text");
        assert_eq!(parse_html(s.to_string()), comp);
    }

    #[test]
    #[should_panic]
    fn test_html_parse_bad() {}
}
