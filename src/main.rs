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

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let mut r = BufReader::new(File::open(opt.path)?);

    identifiers::init();
    for identifier in identifiers::all() {
        r.seek(SeekFrom::Start(0))?;
        match identifier(&mut r) {
            Some(result) => println!("{}", result),
            None => continue,
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::{
        fs::File,
        io::{BufReader, Seek, SeekFrom, Write},
    };
    use tempfile::tempfile;

    pub(crate) fn file(content: &str) -> Result<BufReader<File>> {
        let mut f = tempfile()?;
        f.write(content.as_bytes())?;
        f.seek(SeekFrom::Start(0))?;
        Ok(BufReader::new(f))
    }

    pub(crate) fn assert_match(identifier: identifiers::Identifier, content: &str) -> Result<()> {
        let mut f = file(content)?;
        assert_eq!(identifier(&mut f).is_some(), true);
        Ok(())
    }

    pub(crate) fn assert_no_match(
        identifier: identifiers::Identifier,
        content: &str,
    ) -> Result<()> {
        let mut f = file(content)?;
        assert_eq!(identifier(&mut f).is_some(), false);
        Ok(())
    }
}
