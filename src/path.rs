use crate::{Error, HARDEND};

pub struct BIP32Path(pub(crate) Vec<u32>);

impl BIP32Path {
    pub fn from(path: &str) -> Result<BIP32Path, Error> {
        let mut paths = Vec::new();
        let path = path.replace("/", "\n");
        let mut lines = path.lines();

        while let Some(p) = lines.next() {
            if p.ends_with("H") {
                let index: u32 = p[..p.len()-1].parse().map_err(|_| Error::InvalidIndex)?;
                paths.push(index + HARDEND);
            } else {
                let index: u32 = p.parse().map_err(|_| Error::InvalidIndex)?;
                paths.push(index);
            }
        }

        Ok(BIP32Path(paths))
    }
}