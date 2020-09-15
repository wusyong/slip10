use crate::{Error, HARDEND};

use std::convert::From;

/// A path structure defined by BIP 32.
#[derive(Debug, Eq, PartialEq)]
pub struct BIP32Path(pub(crate) Vec<u32>);

impl BIP32Path {
    /// Create a BIP32Path form string literals.
    pub fn from_str(path: &str) -> Result<BIP32Path, Error> {
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

    /// Return the depth of the BIP32Path. For example, "m/0'/0'" will have depth of 2.
    pub fn depth(&self) -> u8 {
        self.0.len() as u8
    }

    /// Return the index value of corresponding depth in the BIP32Path.
    pub fn index(&self, depth: u8) -> Option<&u32> {
        self.0.get(depth as usize)
    }
}

impl From<Vec<u32>> for BIP32Path {
    fn from(vector: Vec<u32>) -> Self {
        BIP32Path(vector)
    }
}
