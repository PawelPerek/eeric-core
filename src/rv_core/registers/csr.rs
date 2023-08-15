#[derive(Clone)]
pub struct CsrRegisters([u64; 4096]);

impl Default for CsrRegisters {
    fn default() -> Self {
        Self([0; 4096])
    }
}

impl std::ops::Index<usize> for CsrRegisters {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for CsrRegisters {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
