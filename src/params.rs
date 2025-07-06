use std::{iter::Peekable, str::Chars};

use crate::error::Error;

pub struct Params {
    pub symbol: char,
    pub specifier: char,
    pub width: Option<u32>,
    pub reverse: bool,
}

impl Params {
    pub fn parse_from(chars: &mut Peekable<Chars<'_>>) -> Result<Option<Self>, Error> {
        let Some(symbol) = chars.peek().copied().filter(|ch| matches!(ch, '%' | '@')) else {
            return Ok(None);
        };
        chars.next();

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
            return Err(Error::TrailingSymbol(symbol));
        };

        Ok(Some(Self {
            symbol,
            specifier,
            width,
            reverse,
        }))
    }
}
