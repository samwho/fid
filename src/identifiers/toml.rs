use std::io::{BufRead, Read, Seek};

pub(crate) fn identify<T>(input: &mut T) -> Option<String>
where
    T: BufRead + Read + Seek,
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
    fn test_is() -> Result<()> {
        assert_match(
            identify,
            "
            [package]
            name = \"fid\"
            version = \"0.1.0\"
            authors = [\"Sam Rose <hello@samwho.dev>\"]
            edition = \"2018\"
        ",
        )
    }

    #[test]
    fn test_not() -> Result<()> {
        assert_no_match(identify, "This is not a TOML file.")
    }

    #[test]
    fn test_empty() -> Result<()> {
        assert_no_match(identify, "")
    }
}
