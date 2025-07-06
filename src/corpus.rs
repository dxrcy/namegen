use std::fs;
use std::io::{self, BufRead as _, BufReader};
use std::path::{Path, PathBuf};

use rand::Rng;

type CorpusList = Vec<String>;

macro_rules! corpus {
    ( $( $name:ident ),* $(,)? ) => {
        pub struct Corpus {
            directory: PathBuf,
            $( $name: Option<CorpusList>, )*
        }

        impl Corpus {
            pub fn new(directory: PathBuf) -> Self {
                Self {
                    directory,
                    $( $name: None, )*
                }
            }

            $(
                pub fn $name(&mut self, rng: &mut impl Rng) -> io::Result<&str> {
                    random_entry(
                        rng,
                        &self.directory,
                        stringify!($name),
                        &mut self.$name,
                    )
                }
            )*
        }
    };
}

corpus![noun, adjective, color];

fn random_entry<'a>(
    rng: &mut impl Rng,
    directory: &PathBuf,
    name: &str,
    list: &'a mut Option<CorpusList>,
) -> io::Result<&'a str> {
    if list.is_none() {
        *list = Some(read_file(directory, name)?);
    }
    let list = list.as_mut().unwrap();

    assert!(!list.is_empty());
    let index = rng.random_range(0..list.len());
    Ok(&list[index])
}

fn read_file(directory: impl AsRef<Path>, name: &str) -> io::Result<CorpusList> {
    let path = directory.as_ref().join(name);
    let file = fs::OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(file);

    let mut entries = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() {
            entries.push(line);
        }
    }
    Ok(entries)
}
