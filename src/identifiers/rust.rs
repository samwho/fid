use std::io::{BufRead, Read, Seek};

pub(crate) fn rust<T>(input: &mut T) -> Option<String>
where
    T: BufRead + Read + Seek,
{
    let len = input.stream_len().ok()?;
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
    fn test_is_rust_file() -> Result<()> {
        let mut f = file(
            "
            fn main() {
                println!(\"Hello, world!\");
            }
        ",
        )?;

        assert_eq!(rust(&mut f).is_some(), true);
        Ok(())
    }

    #[test]
    fn test_is_not_rust_file() -> Result<()> {
        let mut f = file(
            "
            This is not a Rust file.
        ",
        )?;

        assert_eq!(rust(&mut f).is_some(), false);
        Ok(())
    }

    #[test]
    fn test_empty_file() -> Result<()> {
        let mut f = file("")?;
        assert_eq!(rust(&mut f).is_some(), false);
        Ok(())
    }
}
