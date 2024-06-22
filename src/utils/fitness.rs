use crate::utils::types;

/// Determines if `word` is a compound word of any words in `map` with length greater than `min_len`.
/// If `min_len` is none, it defaults to 6. Returns `Some(match)` with a borrow of the first match (greedy),
/// else None.
pub fn word_is_compound<'a>(
    word: &'a str,
    map: &types::Dict,
    min_len: Option<usize>,
) -> Option<&'a str> {
    let min = min_len.unwrap_or(6);

    if word.len() < min {
        return None;
    }

    // scans in a pyramid up, i.e. scans substr of len min, min+1 ... word.len() - 1

    let mut len = min;

    while len < word.len() {
        let mut begin = 0;
        let mut end = len;

        while end <= word.len() {
            if map.contains(&word[begin..end]) {
                return Some(&word[begin..end]);
            }

            begin += 1;
            end += 1;
        }

        len += 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    // TODO make this const ...
    fn get_dict() -> types::Dict {
        From::from([
            "first".to_string(),
            "second".to_string(),
            "third".to_string(),
            "fifteenth".to_string(),
        ])
    }

    #[test]
    fn test_word_is_compound() {
        let dict = get_dict();
        assert!(word_is_compound("wordsecond", &dict, None).is_some());
        assert!(word_is_compound("wordsecondword", &dict, None).is_some());
        assert!(word_is_compound("wordsecond", &dict, None).is_some());
    }

    #[test]
    fn test_short_words_not_compound() {
        let dict = get_dict();
        assert!(word_is_compound("first", &dict, None).is_none());
        assert!(word_is_compound("firstword", &dict, None).is_none());
        assert!(word_is_compound("firstword", &dict, Some(4)).is_some());
        assert!(word_is_compound("reallybigword", &dict, None).is_none());
    }

    #[test]
    fn test_self_not_compounds() {
        let dict = get_dict();
        assert!(word_is_compound("fifteenth", &dict, None).is_none());
    }

    #[test]
    fn test_long_match() {
        let dict = get_dict();
        let now = Instant::now();
        const LEN: usize = 30;
        assert!(word_is_compound(&"a".repeat(LEN), &dict, None).is_none());
        println!(
            "processed word of length {LEN} in {}us",
            now.elapsed().as_micros()
        );
        // 250us for a long word is permissible on debug build.
        assert!(now.elapsed().as_micros() < 250);
    }
}
