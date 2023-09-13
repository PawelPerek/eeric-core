use super::snapshot::Snapshotable;

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Memory {
    raw: Vec<u8>,
    data_ptr: usize,
}

impl Snapshotable for Memory {
    type Snapshot = Vec<u8>;

    fn snapshot(&self) -> Self::Snapshot {
        self.raw.clone()
    }
}

impl Memory {
    fn new(raw: Vec<u8>) -> Self {
        let data_ptr = raw.len();
        Self { raw, data_ptr }
    }

    pub fn get<const BYTES: usize>(&self, address: usize) -> [u8; BYTES] {
        let mut bytes = [0; BYTES];

        bytes[..BYTES].copy_from_slice(&self.raw[address..(BYTES + address)]);

        bytes
    }

    pub fn fallible_get<const BYTES: usize>(&self, address: usize) -> Option<[u8; BYTES]> {
        let mut bytes = [0; BYTES];

        for (offset, byte_element) in bytes.iter_mut().enumerate().take(BYTES) {
            let Some(byte) = self.raw.get(address + offset).cloned() else {
                return None;
            };

            *byte_element = byte;
        }

        Some(bytes)
    }

    pub fn set<const BYTES: usize>(&mut self, address: usize, value: [u8; BYTES]) {
        self.raw[address..(BYTES + address)].copy_from_slice(&value[..BYTES]);
    }

    pub fn assign(&mut self, data: &[u8]) {
        self.raw[self.data_ptr - data.len()..self.data_ptr].copy_from_slice(data);
        self.data_ptr -= data.len();
    }
}

impl From<Vec<u8>> for Memory {
    fn from(bytes: Vec<u8>) -> Self {
        Self::new(
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
        Self::new(vec![0; 0x1000])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_works() {
        let mut mem = Memory::new(vec![0; 0x10]);

        assert_eq!(mem.data_ptr, 0x10);

        mem.assign(&[1, 2]);

        assert_eq!(mem.data_ptr, 0xE);
        assert_eq!(mem.raw, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2]);

        mem.assign(&[5, 10, 15]);

        assert_eq!(mem.data_ptr, 0xB);
        assert_eq!(mem.raw, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 15, 1, 2]);
    }
}
