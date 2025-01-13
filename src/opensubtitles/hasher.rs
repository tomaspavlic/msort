use std::fmt::Display;
use std::io::{Read, Seek, SeekFrom};
use std::{fs::File, path::Path};

const SIZE: usize = size_of::<usize>();
const N: i64 = 0x10000;

#[derive(Debug, PartialEq, Eq)]
pub struct MovieHash([u8; 8]);

impl Display for MovieHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.0 {
            write!(f, "{:02x}", b)?;
        }

        Ok(())
    }
}

pub fn compute_moviehash(path: &Path) -> std::io::Result<MovieHash> {
    let file = File::open(path)?;
    let file_len = file.metadata()?.len() as i64;

    compute(file, file_len)
}

fn compute<S>(mut stream: S, file_len: i64) -> std::io::Result<MovieHash>
where
    S: Read + Seek,
{
    let mut i = 0i64;
    let mut lhash = file_len;
    let mut buf = [0u8; SIZE];

    while i < N / SIZE as i64 && stream.read_exact(&mut buf).is_ok() {
        i += 1;
        let a = i64::from_ne_bytes(buf);
        lhash = lhash.wrapping_add(a);
    }

    i = 0;
    let p = (file_len - N).max(0);
    stream.seek(SeekFrom::Start(p as u64))?;
    while i < N / SIZE as i64 && stream.read_exact(&mut buf).is_ok() {
        i += 1;
        let a = i64::from_ne_bytes(buf);
        lhash = lhash.wrapping_add(a);
    }

    let mut b = lhash.to_ne_bytes();
    b.reverse();

    Ok(MovieHash(b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn compute_works() {
        let file_content = "this can be some video file but it is UTF-8 string for testing.";
        // just to avoid having to read actual file in tests put the string buffer into
        // something that implement Seek trait.
        let content = Cursor::new(file_content);
        let hash = compute(content, file_content.len() as i64).unwrap();

        assert_eq!(hash, MovieHash([24, 151, 125, 98, 16, 45, 108, 179]));
    }

    #[test]
    fn moviehash_format_is_correct() {
        let movie_hash = MovieHash([24, 151, 125, 98, 16, 45, 108, 179]);
        let fmt = movie_hash.to_string();

        assert_eq!(fmt, "18977d62102d6cb3");
    }
}
