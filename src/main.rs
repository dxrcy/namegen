#![allow(clippy::uninlined_format_args)]

mod args;
mod corpus;
mod error;
mod params;

use std::io;
use std::path::PathBuf;

use clap::Parser;
use rand::Rng;

use self::corpus::Corpus;
use self::error::Error;
use self::params::Params;

fn main() {
    let args = args::Args::parse();
    let mut corpus = Corpus::new(PathBuf::from("corpus"));
    let mut stdout = io::stdout();
    let mut rng = rand::rng();

    display(&mut stdout, &mut rng, &mut corpus, &args.format).unwrap();
}

fn display(
    w: &mut impl io::Write,
    rng: &mut impl Rng,
    corpus: &mut Corpus,
    format: &str,
) -> Result<(), Error> {
    let mut chars = format.chars();

    while let Some(ch) = chars.next() {
        if ch != '%' {
            write!(w, "{}", ch)?;
            continue;
        }

        let params = Params::parse_from(&mut chars)?;

        let word = match params.specifier {
            '%' => {
                write!(w, "{}", ch)?;
                continue;
            }

            'N' => corpus.noun(rng)?,
            'A' => corpus.adjective(rng)?,
            'C' => corpus.color(rng)?,

            'd' | 'x' | 'X' => {
                for _ in 0..params.width.unwrap_or(1) {
                    match params.specifier {
                        'd' => write!(w, "{}", rng.random_range(0..10))?,
                        'x' => write!(w, "{:x}", rng.random_range(0..16))?,
                        'X' => write!(w, "{:X}", rng.random_range(0..16))?,
                        _ => unreachable!(),
                    }
                }
                continue;
            }

            specifier => return Err(Error::UnknownSpecifier(specifier)),
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
