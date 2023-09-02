use super::snapshot::Snapshotable;

#[derive(Clone)]
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

        for offset in 0..BYTES {
            bytes[offset] = self.0[address + offset];
        }

        bytes
    }

    pub fn fallible_get<const BYTES: usize>(&self, address: usize) -> Option<[u8; BYTES]> {
        let mut bytes = [0; BYTES];

        for offset in 0..BYTES {
            let Some(byte) = self.0.get(address + offset).cloned() else {
                return None;
            };

            bytes[offset] = byte;
        }

        Some(bytes)
    }

    pub fn set<const BYTES: usize>(&mut self, address: usize, value: [u8; BYTES]) {
        for offset in 0..BYTES {
            self.0[address + offset] = value[offset];
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self(vec![0; 0x1000])
    }
}
