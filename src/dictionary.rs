use std::collections::HashMap;

use rq;
use serde::{Deserialize, Serialize};
// use serde_json::Result;

struct WiktionaryDefinition {
    definition: String,
    examples: Vec<String>,
}

struct WiktionaryUsage {
    partOfSpeech: String,
    definitions: Vec<WiktionaryDefinition>,
}

#[derive(Deserialize)]
struct WiktionaryResponse {
    usages: Vec<WiktionaryUsage>,
}

struct Defintion {
    word: String,
    definition: String,
}

fn get_rq(word: &str) -> Option<Defintion> {
    let url = format!(
        "https://en.wiktionary.org/api/rest_v1/page/definition/{}",
        word
    );

    let full_url = rq::Url::parse_with_params(&url, &[("redirect", "false")]).ok()?;

    let result = rq::blocking::get(full_url).ok()?;

    let mut map = HashMap::new();
    map.insert("partOfSpeech", "");

    let json = result.json();

    let d = Defintion {
        word: String::from(word),
        definition: String::new(),
    };

    Some(d)
}
