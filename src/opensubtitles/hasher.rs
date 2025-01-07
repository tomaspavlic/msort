use std::fmt::Display;
use std::io::{Read, Seek, SeekFrom};
use std::{fs::File, path::Path};

const SIZE: usize = size_of::<usize>();
const N: i64 = 0x10000;

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
    let mut file = File::open(path)?;
    let file_len = file.metadata()?.len() as i64;
    let mut i = 0i64;
    let mut lhash = file_len;
    let mut buf = [0u8; SIZE];

    while i < N / SIZE as i64 && file.read_exact(&mut buf).is_ok() {
        i += 1;
        let a = i64::from_ne_bytes(buf);
        lhash = lhash.wrapping_add(a);
    }

    i = 0;
    let p = (file_len - N).max(0);
    file.seek(SeekFrom::Start(p as u64))?;
    while i < N / SIZE as i64 && file.read_exact(&mut buf).is_ok() {
        i += 1;
        let a = i64::from_ne_bytes(buf);
        lhash = lhash.wrapping_add(a);
    }

    let mut b = lhash.to_ne_bytes();
    b.reverse();

    Ok(MovieHash(b))
}
