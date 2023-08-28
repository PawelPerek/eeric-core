#[derive(Clone, Default)]
pub struct FloatRegisters([f64; 32]);

impl FloatRegisters {
    pub fn snapshot(&self) -> [f64; 32] {
        self.0
    }
    
}

impl std::ops::Index<usize> for FloatRegisters {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for FloatRegisters {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
