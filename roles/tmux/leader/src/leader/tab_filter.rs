//! Subsequence fuzzy match for tab names (case-insensitive). Lower score = better match.

/// Greedy left-to-right subsequence match. Returns a score (sum of matched character
/// indices) so tighter / earlier matches sort before looser ones. `None` if no match.
pub(crate) fn fuzzy_match_score(needle: &str, haystack: &str) -> Option<u32> {
    if needle.is_empty() {
        return Some(0);
    }
    let hay: Vec<char> = haystack.chars().flat_map(|c| c.to_lowercase()).collect();
    let mut score = 0u32;
    let mut pos = 0usize;
    for n in needle.chars().flat_map(|c| c.to_lowercase()) {
        let mut found = None;
        for (j, &c) in hay.iter().enumerate().skip(pos) {
            if c == n {
                found = Some(j);
                break;
            }
        }
        let j = found?;
        score += j as u32;
        pos = j + 1;
    }
    Some(score)
}
