#[derive(Clone)]
pub struct Memory(Vec<u8>);

#[derive(Clone, Debug)]
pub struct MemorySnapshot(Vec<u8>);

impl Memory {
    pub fn snapshot(&self) -> MemorySnapshot {
        MemorySnapshot(self.0.clone())
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
