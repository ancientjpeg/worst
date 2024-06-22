use crate::utils::types;

pub fn word_is_compound<T>(word: &str, map: &types::WordMap<T>, min_len: Option<usize>) -> bool {
    let min = min_len.unwrap_or(6);

    if word.len() < min {
        return false;
    }

    // scans in a pyramid up, i.e. scans substr of len min, min+1 ... word.len() - 1

    let mut len = min;

    while len < word.len() {
        let mut begin = 0;
        let mut end = len;

        while end <= word.len() {
            if map.contains_key(&word[begin..end]) {
                return true;
            }

            begin += 1;
            end += 1;
        }

        len += 1;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO make this const ...
    fn get_map() -> types::WordCountMap {
        types::WordCountMap::from([
            ("first".to_string(), 0usize),
            ("second".to_string(), 0usize),
            ("third".to_string(), 0usize),
            ("fifteenth".to_string(), 0usize),
        ])
    }

    #[test]
    fn test_word_is_compound() {
        let map = get_map();
        assert!(word_is_compound("wordsecond", &map, None));
        assert!(word_is_compound("wordsecondword", &map, None));
        assert!(word_is_compound("wordsecond", &map, None));
    }

    #[test]
    fn test_short_words_not_compound() {
        let map = get_map();
        assert!(!word_is_compound("first", &map, None));
        assert!(!word_is_compound("firstword", &map, None));
        assert!(word_is_compound("firstword", &map, Some(4)));
        assert!(!word_is_compound("reallybigword", &map, None));
    }

    #[test]
    fn test_self_not_compounds() {
        let map = get_map();
        assert!(!word_is_compound("fifteenth", &map, None));
    }
}
