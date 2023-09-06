use super::snapshot::Snapshotable;

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Memory(Vec<u8>);

impl Snapshotable for Memory {
    type Snapshot = Vec<u8>;

    fn snapshot(&self) -> Self::Snapshot {
        self.0.clone()
    }
}

impl Memory {
    pub fn get<const BYTES: usize>(&self, address: usize) -> [u8; BYTES] {
        let mut bytes = [0; BYTES];

        bytes[..BYTES].copy_from_slice(&self.0[address..(BYTES + address)]);

        bytes
    }

    pub fn fallible_get<const BYTES: usize>(&self, address: usize) -> Option<[u8; BYTES]> {
        let mut bytes = [0; BYTES];

        for (offset, byte_element) in bytes.iter_mut().enumerate().take(BYTES) {
            let Some(byte) = self.0.get(address + offset).cloned() else {
                return None;
            };

            *byte_element = byte;
        }

        Some(bytes)
    }

    pub fn set<const BYTES: usize>(&mut self, address: usize, value: [u8; BYTES]) {
        self.0[address..(BYTES + address)].copy_from_slice(&value[..BYTES]);
    }
}

impl From<Vec<u8>> for Memory {
    fn from(bytes: Vec<u8>) -> Self {
        Self(
            bytes
                .iter()
                .cloned()
                .chain(std::iter::repeat(0).take(0x1000 - bytes.len()))
                .collect(),
        )
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self(vec![0; 0x1000])
    }
}
