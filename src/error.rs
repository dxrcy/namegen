use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    UnknownSpecifier(char, char),
    TrailingSymbol(char),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IO(error) => write!(f, "{}", error),
            Self::UnknownSpecifier(symbol, specifier) => {
                write!(f, "Unknown specifier '{}{}'", symbol, specifier)
            }
            Self::TrailingSymbol(symbol) => write!(f, "Trailing symbol '{}'", symbol),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::IO(error)
    }
}
