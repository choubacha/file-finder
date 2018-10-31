use rayon::prelude::*;

#[derive(PartialEq, Debug)]
pub struct Match<'a> {
    pub matches: Vec<usize>,
    pub score: f64,
    pub string: &'a str,
}

pub fn find<'a, T: AsRef<str> + 'a + Sync>(needle: &str, stack: &'a [T]) -> Vec<Match<'a>> {
    let mut matches: Vec<Match<'a>> = stack.par_iter().filter_map(|hay| score(needle, hay)).collect();
    // TODO: Refactor unwrap
    matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    matches
}

/// Score will match the needle against the stack and score it based on a few criteria:
///
/// 1. The closer successive letters are, the higher the score (max: 5)
/// 2. Every matched character is 1 point
/// 3. The ratio for the score should favors prefix matches (or those closer to the left)
pub fn score<'a, T: AsRef<str>>(needle: &str, haystack: &'a T) -> Option<Match<'a>> {
    let mut haystack_chars = haystack.as_ref().chars();
    let mut char_count = 0;
    let mut score = 0;
    let mut matches = Vec::with_capacity(needle.len());

    for needle in needle.chars() {
        // Start with extra points that _might_ be applied
        let mut extra_points = 5;

        loop {
            if let Some(hay) = haystack_chars.next() {
                // Track how many chars we've seen
                char_count += 1;

                if hay == needle {
                    // Found a match, earn a point!
                    score += 1;
                    matches.push((char_count - 1) as usize);

                    break;
                } else {
                    // Found no match, reduce the extrapoints
                    extra_points -= 1;
                }
            } else {
                // Did not find a match
                return None;
            }
        }

        if extra_points > 0 {
            score += extra_points;
        }
    }
    Some(Match {
        score: score as f64 / char_count as f64,
        matches: matches,
        string: haystack.as_ref(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoring_a_string_test() {
        let score = score("de", &"def").unwrap();
        assert_eq!(score.score, 12.0 / 2.0);
    }

    #[test]
    fn finds_no_match() {
        assert_eq!(score("abc", &"def"), None);
        assert_eq!(score("deq", &"def"), None);
    }

    #[test]
    fn find_matches() {
        let matches = find("abc", &["a", "ac", "abc", "defasedbkjfkjc"]);
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].string, "abc");
        assert_eq!(matches[0].matches, [0, 1, 2]);
    }
}
