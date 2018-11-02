#[derive(PartialEq, Debug)]
pub struct Match {
    pub matches: Vec<usize>,
    pub score: f64,
    pub string: String,
}

/// Score will match the needle against the stack and score it based on a few criteria:
///
/// 1. The closer successive letters are, the higher the score (max: 5)
/// 2. Every matched character is 1 point
/// 3. The ratio for the score should favors prefix matches (or those closer to the left)
pub fn score<'a, T: AsRef<str>>(needle: &str, haystack: &'a T) -> Option<Match> {
    let mut haystack_chars = haystack.as_ref().char_indices();
    let mut char_count = 0;
    let mut score = 0;
    let mut matches = Vec::with_capacity(needle.len());

    for needle in needle.chars() {
        // Start with extra points that _might_ be applied
        let mut extra_points = 5i32;

        loop {
            if let Some((index, hay)) = haystack_chars.next() {
                // Track how many chars we've seen
                char_count += 1;

                if hay == needle {
                    // Found a match, earn a point!
                    score += 1;
                    matches.push(index);

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
    let score = score as f64 / char_count as f64;
    if score > 0.5 {
        Some(Match {
            score,
            matches,
            string: haystack.as_ref().to_owned(),
        })
    } else {
        None
    }
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
    fn records_matches_based_on_char_indices() {
        let m = score("abc", &"會意字ab會意字c").unwrap();
        assert_eq!(m.matches, [9, 10, 20])
    }

    #[test]
    fn does_not_match_if_spread_out() {
        let score = score("abc", &"attttttttttttttttttttttbtttttttttttttttttttttttc");
        println!("{:?}", score);
        assert!(score.is_none());
    }
}
