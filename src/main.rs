#![feature(seek_convenience)]

mod identifiers;

use anyhow::Result;
use std::{
    fs::File,
    io::{BufReader, Seek, SeekFrom},
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    path: PathBuf,
}

pub(crate) type Identifier<T> = fn(&mut T) -> Option<String>;

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let fs: Vec<Identifier<BufReader<File>>> = vec![identifiers::rust];

    let mut r = BufReader::new(File::open(opt.path)?);

    for f in fs {
        r.seek(SeekFrom::Start(0))?;
        match f(&mut r) {
            Some(result) => println!("{}", result),
            None => continue,
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::{Write, BufReader, SeekFrom, Seek}};
    use tempfile::tempfile;
    use anyhow::Result;

    pub(crate) fn file(content: &str) -> Result<BufReader<File>> {
        let mut f = tempfile()?;
        f.write(content.as_bytes())?;
        f.seek(SeekFrom::Start(0))?;
        Ok(BufReader::new(f))
    }
}