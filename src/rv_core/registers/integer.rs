#[derive(Clone, Default)]
pub struct IntegerRegisters([u64; 31]);

impl std::ops::Index<usize> for IntegerRegisters {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &0,
            _ => &self.0[index - 1]
        }
    }
}

impl std::ops::IndexMut<usize> for IntegerRegisters {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut 0,
            _ => &mut self.0[index - 1]
        }
    }
}
