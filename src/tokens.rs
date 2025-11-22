use once_cell::sync::Lazy;
use tiktoken_rs::CoreBPE;

static TOKENIZER: Lazy<CoreBPE> = Lazy::new(|| tiktoken_rs::p50k_base().unwrap());

pub fn count_tokens(text: &str) -> usize {
    TOKENIZER.encode_with_special_tokens(text).len()
}

pub fn count_tokens_from_bytes(bytes: &[u8]) -> usize {
    match std::str::from_utf8(bytes) {
        Ok(text) => count_tokens(text),
        Err(_) => {
            let text = String::from_utf8_lossy(bytes);
            count_tokens(&text)
        }
    }
}
