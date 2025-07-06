#![allow(clippy::uninlined_format_args)]

mod args;
mod corpus;
mod error;
mod params;

use std::io::{self, BufWriter};

use chrono::{DateTime, Local};
use clap::Parser;
use rand::Rng;

use self::corpus::Corpus;
use self::error::Error;
use self::params::Params;

fn main() {
    let args = args::Args::parse();

    let directory = args.corpus.unwrap_or_else(|| {
        let parent =
            dirs_next::data_dir().expect("Failed to find 'data' directory (XDG_DATA_HOME)");
        parent.join("namegen")
    });

    let mut corpus = Corpus::new(directory);
    let mut stdout = BufWriter::new(io::stdout());
    let mut rng = rand::rng();
    let date = Local::now();

    if let Err(error) = display(&mut stdout, &mut rng, &mut corpus, &date, &args.format) {
        eprintln!("namegen: {}", error);
    }
}

fn display(
    w: &mut impl io::Write,
    rng: &mut impl Rng,
    corpus: &mut Corpus,
    date: &DateTime<Local>,
    format: &str,
) -> Result<(), Error> {
    let mut chars = format.chars().peekable();

    while let Some(ch) = chars.peek().copied() {
        let Some(params) = Params::parse_from(&mut chars)? else {
            write!(w, "{}", ch)?;
            chars.next();
            continue;
        };

        let word = match (params.symbol, params.specifier) {
            (_, '%' | '@') => {
                write!(w, "{}", params.specifier)?;
                continue;
            }

            ('%', 'N') => corpus.get("noun", rng)?,
            ('%', 'A') => corpus.get("adjective", rng)?,
            ('%', 'C') => corpus.get("color", rng)?,

            ('%', 'd' | 'x' | 'X' | 'l' | 'L') => {
                for _ in 0..params.width.unwrap_or(1) {
                    match params.specifier {
                        'd' => write!(w, "{}", rng.random_range(0..10))?,
                        'x' => write!(w, "{:x}", rng.random_range(0..16))?,
                        'X' => write!(w, "{:X}", rng.random_range(0..16))?,
                        'l' => write!(w, "{}", rng.random_range('a'..='z'))?,
                        'L' => write!(w, "{}", rng.random_range('A'..='Z'))?,
                        _ => unreachable!(),
                    }
                }
                continue;
            }

            ('@', specifier) if is_date_specifier(specifier) => {
                let mut buffer = [0u8; 2];
                let format = date.format(date_format(&mut buffer, specifier));
                write!(w, "{}", format)?;
                continue;
            }

            (symbol, specifier) => return Err(Error::UnknownSpecifier(symbol, specifier)),
        };

        if params.reverse {
            write!(w, "{}", word)?;
        }
        if let Some(width) = params.width {
            for _ in word.len()..width as usize {
                write!(w, ".")?;
            }
        }
        if !params.reverse {
            write!(w, "{}", word)?;
        }
    }

    writeln!(w)?;
    Ok(())
}

#[rustfmt::skip]
fn is_date_specifier(ch: char) -> bool {
    matches!(
        ch,
        'Y' | 'C' | 'y' | 'q' | 'm' | 'b' | 'B' | 'd' | 'a' | 'A' | 'w' | 'u' | 'U'
            | 'W' | 'G' | 'g' | 'V' | 'j' | 'D' | 'x' | 'F' | 'v' | 'H' | 'I' | 'P'
            | 'p' | 'M' | 'S' | 'f' | 'R' | 'T' | 'X' | 'r' | 'Z' | 'z' | '+' | 's'
    )
}

fn date_format(buffer: &mut [u8; 2], specifier: char) -> &str {
    assert!(specifier.is_ascii());
    buffer[0] = b'%';
    buffer[1] = specifier as u8;
    // SAFETY: All items of buffer are known to be valid ASCII, and thus valid UTF-8
    unsafe { str::from_utf8_unchecked(buffer) }
}
