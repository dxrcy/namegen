use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    pub format: String,

    #[arg(long)]
    pub corpus: Option<PathBuf>,
}
