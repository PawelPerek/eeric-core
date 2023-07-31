#[derive(Clone)]
struct Memory(Vec<u8>);

impl Memory {
    fn get(&self, address: usize) -> u8 {
        self.0[address]
    }

    fn set(&mut self, address: usize, value: u8) {
        self.0[address] = value;
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self(vec![0; 0x1000])
    }
}
