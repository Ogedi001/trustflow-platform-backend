//! String utilities
//!
//! Common string manipulation and formatting utilities.

/// String utility functions
pub struct StringUtils;

impl StringUtils {
    /// Truncate string to maximum length, adding ellipsis if truncated
    pub fn truncate(s: &str, max_len: usize) -> String {
        if s.len() > max_len {
            format!("{}...", &s[..max_len.saturating_sub(3)])
        } else {
            s.to_string()
        }
    }

    /// Capitalize first letter
    pub fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().to_string() + chars.as_str(),
        }
    }

    /// Convert to title case (capitalize each word)
    pub fn title_case(s: &str) -> String {
        s.split_whitespace()
            .map(Self::capitalize)
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Remove leading and trailing whitespace, and collapse internal whitespace
    pub fn normalize_whitespace(s: &str) -> String {
        s.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// Check if string is only whitespace
    pub fn is_blank(s: &str) -> bool {
        s.trim().is_empty()
    }

    /// Remove all whitespace
    pub fn remove_whitespace(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }

    /// Mask sensitive information, keeping first and last N characters
    pub fn mask(s: &str, show_first: usize, show_last: usize) -> String {
        if s.len() <= show_first + show_last {
            return "*".repeat(s.len());
        }

        let first = &s[..show_first];
        let last = &s[s.len() - show_last..];
        let masked = "*".repeat(s.len() - show_first - show_last);

        format!("{}{}{}", first, masked, last)
    }

    /// Split string respecting quoted sections
    pub fn split_respecting_quotes(s: &str, delimiter: char, quote: char) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            if c == quote {
                in_quotes = !in_quotes;
            } else if c == delimiter && !in_quotes {
                result.push(current.trim().to_string());
                current.clear();
            } else {
                current.push(c);
            }
        }

        if !current.is_empty() {
            result.push(current.trim().to_string());
        }

        result
    }

    /// Similarity score between two strings (0.0 to 1.0)
    pub fn similarity(s1: &str, s2: &str) -> f64 {
        let s1 = s1.to_lowercase();
        let s2 = s2.to_lowercase();

        if s1.is_empty() && s2.is_empty() {
            return 1.0;
        }

        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }

        // Levenshtein distance based similarity
        let distance = levenshtein_distance(&s1, &s2);
        let max_len = s1.len().max(s2.len());
        1.0 - (distance as f64 / max_len as f64)
    }
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }

    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(
                    matrix[i][j + 1] + 1,      // deletion
                    matrix[i + 1][j] + 1,      // insertion
                ),
                matrix[i][j] + cost,           // substitution
            );
        }
    }

    matrix[len1][len2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate() {
        assert_eq!(StringUtils::truncate("hello world", 5), "he...");
        assert_eq!(StringUtils::truncate("hi", 5), "hi");
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(StringUtils::capitalize("hello"), "Hello");
    }

    #[test]
    fn test_title_case() {
        assert_eq!(StringUtils::title_case("hello world"), "Hello World");
    }

    #[test]
    fn test_mask() {
        let masked = StringUtils::mask("1234567890", 2, 2);
        assert_eq!(masked, "12******90");
    }

    #[test]
    fn test_similarity() {
        assert_eq!(StringUtils::similarity("abc", "abc"), 1.0);
        assert!(StringUtils::similarity("abc", "abd") > 0.5);
    }
}
