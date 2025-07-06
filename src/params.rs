use std::str::Chars;

use crate::error::Error;

pub struct Params {
    pub specifier: char,
    pub width: Option<u32>,
    pub reverse: bool,
}

impl Params {
    pub fn parse_from(chars: &mut Chars<'_>) -> Result<Self, Error> {
        // This function must consume peeked char before returning `Ok`.
        // This happens when consuming specifier
        let mut chars = chars.peekable();

        let mut width = None;
        let mut reverse = false;

        if chars.peek().copied().is_some_and(|ch| ch == '-') {
            chars.next();
            reverse = true;
        }

        while let Some(ch) = chars.peek().copied().filter(char::is_ascii_digit) {
            chars.next();
            let digit = ch as u32 - b'0' as u32;
            let width = width.get_or_insert(0);
            *width *= 10;
            *width += digit;
        }

        let Some(specifier) = chars.next() else {
            return Err(Error::TrailingSymbol);
        };

        Ok(Self {
            specifier,
            width,
            reverse,
        })
    }
}
