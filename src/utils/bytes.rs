use std::fmt;
use std::vec;
use std::borrow::Cow;
use std::error;

use encoding_rs::Encoding;
use memchr;

const FIRST_FEW_BYTES: usize = 8000;

#[derive(Debug)]
pub enum DecodingError {
    /// BOM was present, but could not decode file using the specified encoding.
    InvalidBom
}

impl error::Error for DecodingError {
}

impl fmt::Display for DecodingError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecodingError::InvalidBom => write!(fmt, "invalid bom"),
        }
    }
}

/// Test if file is a binary file by checking if any of `FIRST_FEW_BYTES` first bytes are null
/// bytes (`0x00`).
pub fn is_binary(bytes: &[u8]) -> bool {
    // This mimics the git test that does the same from here:
    // https://github.com/git/git/blob/2d3b1c576c85b7f5db1f418907af00ab88e0c303/xdiff-interface.c#L202
    let end = usize::min(FIRST_FEW_BYTES, bytes.len());
    memchr::memchr(0u8, &bytes[..end]).is_some()
}

/// Do your best to try and construct a Bytes instance while performing as much detection as
/// possible.
///
/// This looks at:
/// * The BOM of the file, if present.
pub fn decode<'a>(bytes: &'a [u8]) -> Result<Cow<'a, [u8]>, DecodingError> {
    if bytes.len() >= 2 {
        let end = usize::min(3, bytes.len());

        if let Some((encoding, n)) = Encoding::for_bom(&bytes[..end]) {
            let out = match encoding.decode_without_bom_handling_and_without_replacement(&bytes[n..]) {
                None => return Err(DecodingError::InvalidBom),
                Some(out) => out,
            };

            match out {
                Cow::Borrowed(s) => return Ok(Cow::from(s.as_bytes())),
                Cow::Owned(s) => return Ok(Cow::from(s.into_bytes())),
            }
        }
    }

    // TODO: look for encoding comments in the first 5 (ish) lines.

    // encoding could not be detected, pass through.
    Ok(Cow::Borrowed(bytes))
}

#[derive(Clone, Copy)]
pub struct Bytes<'a> {
    bytes: &'a [u8],
}

impl<'a> Bytes<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes
        }
    }

    /// Get the length of the bytes contained.
    pub fn len(self) -> usize {
        self.bytes.len()
    }

    /// Treat as a bytes slice.
    pub fn as_bytes(self) -> &'a [u8] {
        self.bytes
    }

    /// Iterate over the bytes as chars.
    pub fn utf8_chars_lossy(self) -> Utf8CharsLossy {
        // NB: if core::lossy was available we could do this as a zero copy.
        // right now, this is the easiest approach.
        let it = String::from_utf8_lossy(&self.bytes)
            .chars()
            .collect::<Vec<char>>().into_iter();

        Utf8CharsLossy {
            it
        }
    }

    /// Remove leading and trailing whitespace.
    ///
    /// Whitespace is identified by using `char::is_whitespace` on the char equivalent of a byte
    /// treated as ascii.
    pub fn trim(self) -> Bytes<'a> {
        let mut b = self.bytes;

        while b.len() > 0 && char::is_whitespace(b[0] as char) {
            b = &b[1..];
        }

        while b.len() > 0 && char::is_whitespace(b[b.len() - 1] as char) {
            b = &b[..(b.len() - 1)];
        }

        Bytes::new(b)
    }

    /// Get an iterator over all lines.
    pub fn lines(self) -> Lines<'a> {
        Lines {
            buf: self.bytes,
        }
    }

    /// Check if the given byte is contained.
    pub fn contains(self, needle: &[u8]) -> bool {
        self.bytes.windows(needle.len()).any(|w| w == needle)
    }

    /// Check if bytes array starts with the given value.
    pub fn starts_with(self, needle: &[u8]) -> bool {
        self.bytes.starts_with(needle)
    }
}

impl<'a> fmt::Display for Bytes<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        String::from_utf8_lossy(self.bytes).fmt(fmt)
    }
}

/// Iterator over UTF-8 characters, replacing non-legal sequences with the unicode replacement
/// character (U+FFFD).
#[derive(Clone)]
pub struct Utf8CharsLossy {
    it: vec::IntoIter<char>,
}

impl Iterator for Utf8CharsLossy {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }
}

#[derive(Clone)]
pub struct Lines<'a> {
    buf: &'a [u8],
}

const NL: u8 = b'\n';
const CR: u8 = b'\r';

impl<'a> Iterator for Lines<'a> {
    type Item = Bytes<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        use std::mem;

        if let Some(mut idx) = memchr::memchr2(NL, CR, self.buf) {
            let o = &self.buf[..idx];

            if self.buf[idx] == NL && self.buf.get(idx + 1).map(|c| *c == CR).unwrap_or(false) {
                idx += 1;
            }

            self.buf = &self.buf[idx + 1..];
            return Some(Bytes::new(o));
        }

        if !self.buf.is_empty() {
            return Some(Bytes::new(mem::replace(&mut self.buf, &[])));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::Bytes;

    #[test]
    fn test_lines() {
        assert_eq!(5, Bytes::new(b"foo\nbar\n\rbaz\r\rtail").lines().count());
    }

    #[test]
    fn test_utf8_chars_lossy() {
        assert_eq!(
            vec!['a', 'b', 'c', 'd'],
            Bytes::new(b"abcd").utf8_chars_lossy().collect::<Vec<_>>()
        );

        // Non-UTF-8 sequences use replacement character.
        assert_eq!(
            vec!['\u{FFFD}'],
            Bytes::new(b"\x8f").utf8_chars_lossy().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_trim() {
        assert_eq!(b"foo", Bytes::new(b"    foo ").trim().as_bytes());
    }

    #[test]
    fn test_contains() {
        assert!(!Bytes::new(b"foobar").contains(b"blarg"));
        assert!(Bytes::new(b"foobar").contains(b"oob"));
    }

    #[test]
    fn test_starts_with() {
        assert!(!Bytes::new(b"foobar").contains(b"baz"));
        assert!(Bytes::new(b"foobar").contains(b"bar"));
    }
}
