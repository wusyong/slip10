use crate::{Error, HARDEND};

#[derive(Debug, Eq, PartialEq)]
pub struct BIP32Path(pub Vec<u32>);

impl BIP32Path {
    pub fn from(path: &str) -> Result<BIP32Path, Error> {
        let mut paths = Vec::new();
        let path = path.replace("/", "\n");
        let mut lines = path.lines();

        while let Some(p) = lines.next() {
            if p != "m" {
                if p.ends_with("H") || p.ends_with("'") {
                    let index: u32 = p[..p.len() - 1].parse().map_err(|_| Error::InvalidIndex)?;
                    if index < HARDEND {
                        paths.push(index + HARDEND);
                    } else {
                        return Err(Error::InvalidIndex);
                    }    
                } else {
                    let index: u32 = p.parse().map_err(|_| Error::InvalidIndex)?;
                    if index < HARDEND {
                        paths.push(index);
                    } else {
                        return Err(Error::InvalidIndex);
                    } 
                }
            }
        }

        Ok(BIP32Path(paths))
    }
}
