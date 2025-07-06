use std::fs;
use std::io::{self, BufRead as _, BufReader};
use std::path::{Path, PathBuf};

use rand::Rng;

type CorpusList = Vec<String>;

pub struct Corpus {
    directory: PathBuf,
    lists: Vec<(&'static str, CorpusList)>,
}

impl Corpus {
    pub fn new(directory: PathBuf) -> Self {
        Self {
            directory,
            lists: Vec::new(),
        }
    }

    pub fn get(&mut self, name: &'static str, rng: &mut impl Rng) -> io::Result<&str> {
        // TODO(feat): Handle empty list (empty file)
        let list = self.get_list(name)?;
        assert!(!list.is_empty(), "empty file");
        let index = rng.random_range(0..list.len());
        Ok(&list[index])
    }

    fn get_list(&mut self, name: &'static str) -> io::Result<&CorpusList> {
        // Current (stable) borrow rules prevent this from being written nicer
        for i in 0..self.lists.len() {
            if self.lists[i].0 == name {
                return Ok(&self.lists[i].1);
            }
        }

        let list = read_file(&self.directory, name)?;
        self.lists.push((name, list));
        let (_, list) = self.lists.last().expect("item was just pushed above");
        Ok(list)
    }
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
