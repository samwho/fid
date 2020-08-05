
use std::{
    io::{Read, Seek, BufRead},
};

pub(crate) fn toml<T>(input: &mut T) -> Option<String> 
where T: BufRead + Read + Seek
{
    let len = input.stream_len().ok()?;
    if len > 1024 * 1024 || len == 0 {
        return None;
    }

    let mut s = String::new();
    input.read_to_string(&mut s).ok()?;
    let _: toml::Value = s.parse().ok()?;
    Some("TOML file".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use anyhow::Result;

    #[test]
    fn test_is_rust_file() -> Result<()> {
        let mut f = file("
            [package]
            name = \"fid\"
            version = \"0.1.0\"
            authors = [\"Sam Rose <hello@samwho.dev>\"]
            edition = \"2018\"
        ")?;

        assert_eq!(toml(&mut f).is_some(), true);
        Ok(())
    }

    #[test]
    fn test_is_not_rust_file() -> Result<()> {
        let mut f = file("
            This is not a TOML file.
        ")?;

        assert_eq!(toml(&mut f).is_some(), false);
        Ok(())
    }

    #[test]
    fn test_empty_file() -> Result<()> {
        let mut f = file("")?;
        assert_eq!(toml(&mut f).is_some(), false);
        Ok(())
    }
}