use std::io::{BufRead, Read, Seek};
use super::{length};

pub(crate) fn identify<T>(input: &mut T) -> Option<String>
where
    T: BufRead + Read + Seek,
{
    let len = length(input).ok()?;
    if len > 1024 * 1024 || len == 0 {
        return None;
    }

    let mut s = String::new();
    input.read_to_string(&mut s).ok()?;
    let _: syn::File = syn::parse_str(&s).ok()?;
    Some("Rust source code".to_owned())
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
            fn main() {
                println!(\"Hello, world!\");
            }
        ",
        )
    }

    #[test]
    fn test_not() -> Result<()> {
        assert_no_match(identify, "This is not a Rust file.")
    }

    #[test]
    fn test_empty() -> Result<()> {
        assert_no_match(identify, "")
    }
}
