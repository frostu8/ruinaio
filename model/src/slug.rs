//! Slug utilities

use std::fmt::{self, Debug, Display, Formatter};
use std::borrow::Cow;

/// Turns a generic string into a slug.
///
/// This involves removing any invalid punctuation, capitalizing the first
/// letter and any letters directly after any invalid characters. Slashes are
/// also allowed.
pub fn slugify<'a>(s: &'a str) -> Result<Cow<'a, str>, Error> {
    if s.is_empty() {
        return Err(Error::Empty);
    }

    let mut start = 0;
    let mut result = None::<String>;

    while start < s.len() {
        // capitalize first letter if letter is valid
        if let Some(ch) = s[start..].chars().next() {
            if is_valid(ch) {
                if ch.is_ascii_alphabetic() && !ch.is_ascii_uppercase() {
                    // capitalize letter
                    push_iter(&mut result, ch.to_uppercase());

                    start += ch.to_uppercase().count();

                    if start >= s.len() {
                        break;
                    }
                }
            }
        }

        if let Some(idx) = s[start..].chars().position(|ch| !is_valid(ch)) {
            // there are invalid characters! copy up until the characters
            push_str(&mut result, &s[start..start+idx]);

            // skip invalid characters
            start = s[start+idx..].chars().position(is_valid).unwrap_or(s.len()) + start + idx;
        } else {
            // return result
            if let Some(result) = &mut result {
                result.push_str(&s[start..]);
            }

            start = s.len();
        }
    }

    let length = result.as_ref().map(|s| s.len()).unwrap_or(s.len());
    if length > 128 {
        return Err(Error::Length(length));
    }

    match result {
        Some(result) => Ok(Cow::Owned(result)),
        None => Ok(Cow::Borrowed(s)),
    }
}

fn is_valid(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '/'
}

fn push_iter(s: &mut Option<String>, iter: impl Iterator<Item = char>) {
    match s {
        Some(s) => for ch in iter {
            s.push(ch)
        },
        None => *s = Some(iter.collect()),
    }
}

fn push_str(s: &mut Option<String>, n: &str) {
    match s {
        Some(s) => s.push_str(n),
        None => *s = Some(String::from(n)),
    }
}

/// An error for slug parsing.
#[derive(Clone, Debug)]
pub enum Error {
    /// The input string is empty.
    Empty,
    /// The input string is over 128 characters.
    Length(usize),
    /// The input string has an invalid character.
    InvalidChar(char, usize),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::Empty => f.write_str("input string is empty"),
            Error::Length(len) => write!(f, "input string is over 128 chars ({})", len),
            Error::InvalidChar(ch, col) => write!(f, "invalid char '{}' @ col {}", ch, col),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_slugify() {
        use super::slugify;

        assert_eq!(slugify("Help").unwrap(), "Help");
        assert_eq!(slugify("TheWest").unwrap(), "TheWest");

        assert_eq!(slugify("Princess Piggie").unwrap(), "PrincessPiggie");
        assert_eq!(slugify("Three Word Phrase").unwrap(), "ThreeWordPhrase");
        assert_eq!(slugify("lowercase randy").unwrap(), "LowercaseRandy");
        assert_eq!(slugify("three lowercase words").unwrap(), "ThreeLowercaseWords");

        assert_eq!(slugify("Kebab-Case").unwrap(), "KebabCase");
        assert_eq!(slugify("kebab-case").unwrap(), "KebabCase");
        assert_eq!(slugify("Snake_Case").unwrap(), "SnakeCase");
        assert_eq!(slugify("snake_case").unwrap(), "SnakeCase");

        assert_eq!(slugify("The quick brown fox, jumped over the lazy dog.").unwrap(), "TheQuickBrownFoxJumpedOverTheLazyDog");
    }
}

