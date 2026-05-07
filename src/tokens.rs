//! Token counting for LLM context estimation.
//!
//! This module provides token counting using OpenAI's tiktoken library with
//! the `o200k_base` encoding, which is used by modern OpenAI models.
//!
//! Token counts are useful for estimating how much of an LLM's context window
//! your codebase will consume.

use tiktoken_rs::o200k_base_singleton;

/// Count tokens in a string using the o200k_base encoding.
#[inline]
pub fn count_tokens(text: &str) -> usize {
    o200k_base_singleton().encode_with_special_tokens(text).len()
}

/// Count tokens from a byte slice, converting to UTF-8.
///
/// Invalid UTF-8 sequences are replaced with the Unicode replacement character.
#[inline]
pub fn count_tokens_from_bytes(bytes: &[u8]) -> usize {
    match std::str::from_utf8(bytes) {
        Ok(text) => count_tokens(text),
        Err(_) => {
            let text = String::from_utf8_lossy(bytes);
            count_tokens(&text)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_tokens() {
        // Simple test - "Hello, world!" should be a few tokens
        let count = count_tokens("Hello, world!");
        assert!(count > 0 && count < 10);
    }

    #[test]
    fn test_count_tokens_from_bytes() {
        let bytes = b"fn main() { println!(\"Hello\"); }";
        let count = count_tokens_from_bytes(bytes);
        assert!(count > 0);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(count_tokens(""), 0);
    }
}
