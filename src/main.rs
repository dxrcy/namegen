mod args;
mod corpus;
mod error;

use std::io;
use std::path::PathBuf;

use clap::Parser;
use rand::Rng;

use self::corpus::Corpus;
use self::error::Error;

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
        let Some(specifier) = chars.next() else {
            return Err(Error::TrailingSymbol);
        };

        let word = match specifier {
            '%' => {
                write!(w, "{}", ch)?;
                continue;
            }

            'n' => corpus.noun(rng)?,
            'a' => corpus.adjective(rng)?,
            'c' => corpus.color(rng)?,

            _ => return Err(Error::UnknownSpecifier(specifier)),
        };

        write!(w, "{}", word)?;
    }

    writeln!(w)?;
    Ok(())
}
