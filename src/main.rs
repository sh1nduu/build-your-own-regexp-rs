use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let pattern = args.get(1).expect("argument #1 should be pattern");
    let text = args.get(2).expect("argument #2 should be text");

    let result = match search(pattern, text) {
        true => "matches",
        false => "does not matches",
    };
    println!(r#""{}" {} to "{}""#, pattern, result, text);
}

pub fn search(pattern: &str, text: &str) -> bool {
    let (first_pattern, rest_pattern) = split_at(pattern, 1);

    if first_pattern == "^" {
        is_match(rest_pattern, text)
    } else if text.is_empty() {
        is_match(pattern, text)
    } else {
        text.char_indices().into_iter().any(|(idx, _)| {
            let (_, slice) = split_at(text, idx);
            is_match(pattern, slice)
        })
    }
}

fn is_match_one(pattern: &str, text: &str) -> bool {
    if pattern.len() > 1 || text.len() > 1 {
        panic!("pattern or text length exceeds limit")
    }

    if pattern.is_empty() {
        true
    } else if text.is_empty() {
        false
    } else if pattern == "." {
        true
    } else {
        pattern == text
    }
}

fn is_match(pattern: &str, text: &str) -> bool {
    if pattern.is_empty() {
        return true;
    } else if pattern == "$" && text == "" {
        return true;
    }

    match pattern.chars().nth(1) {
        Some(c) if c == '?' => is_match_question(pattern, text),
        Some(c) if c == '*' => is_match_star(pattern, text),
        _ => {
            let (first_pattern, rest_pattern) = split_at(pattern, 1);
            let (first_text, rest_text) = split_at(text, 1);

            is_match_one(first_pattern, first_text) && is_match(rest_pattern, rest_text)
        }
    }
}

fn is_match_question(pattern: &str, text: &str) -> bool {
    let (first_pattern, _) = split_at(pattern, 1);
    let (first_text, text_slice) = split_at(text, 1);
    let (_, pattern_slice) = split_at(pattern, 2);

    (is_match_one(first_pattern, first_text) && is_match(pattern_slice, text_slice))
        || is_match(pattern_slice, text)
}

fn is_match_star(pattern: &str, text: &str) -> bool {
    let (first_pattern, _) = split_at(pattern, 1);
    let (first_text, text_slice) = split_at(text, 1);
    let (_, pattern_slice) = split_at(pattern, 2);

    (is_match_one(first_pattern, first_text) && is_match(pattern, text_slice))
        || is_match(pattern_slice, text)
}

fn split_at(text: &str, size: usize) -> (&str, &str) {
    if text.len() >= size {
        text.split_at(size)
    } else {
        (text, "")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_match_one() {
        assert!(is_match_one("a", "a"));
        assert!(is_match_one("a", "b") == false);
        assert!(is_match_one("", "t"));
        assert!(is_match_one("p", "") == false);
        assert!(is_match_one(".", "z"));
    }

    #[test]
    fn test_is_match() {
        assert!(is_match("a.c", "abc"));
        assert!(is_match("a.c", "bbc") == false);
        assert!(is_match("ab$", "ab"));
        assert!(is_match("ab$", "aba") == false);
        assert!(is_match("ab?c", "ac"));
        assert!(is_match("ab?c", "abc"));
        assert!(is_match("a?b?c?", "abc"));
        assert!(is_match("a?b?c?", ""));
    }

    #[test]
    fn test_search() {
        assert!(search("^abc", "abc"));
        assert!(search("^abcd", "abcd"));
        assert!(search("bc", "abcd"));
        assert!(search("a*", ""));
        assert!(search("a*", "aaaaaa"));
        assert!(search("a*b", "aaaaaab"));
    }
}
