mod vector_engine;
mod vreg;

use vector_engine::VectorEngine;
use vreg::Vreg;

#[derive(Clone)]
pub struct VectorRegisters{
    vregs: [Vreg; 32],
    vec_engine: VectorEngine
}

impl VectorRegisters {
    fn acquire<'a>(&'a self, rs1: usize) -> AcquiredRegisters {
        AcquiredRegisters { rs1: &self.vregs[rs1], rs2: None }
    }

    fn acquire2<'a>(&'a self, rs1: usize, rs2: usize ) -> AcquiredRegisters {
        AcquiredRegisters { rs1: &self.vregs[rs1], rs2: Some(&self.vregs[rs1]) }
    }
}

pub struct AcquiredRegisters<'a>{
    rs1: &'a Vreg,
    rs2: Option<&'a Vreg>
}

impl<'a> AcquiredRegisters<'a> {
    fn execute<F>(&self, builder: F) -> Vreg 
        where
            F: Fn(u64, u64) -> u64 
    {
        // todo: xd
        Vreg(vec![])
    }
}

impl Default for VectorRegisters {
    fn default() -> Self {
        Self { 
            vregs: [(); 32].map(|_| Vreg(vec![])), 
            vec_engine: VectorEngine::new(
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default()
            ).unwrap()
        }
    }
}

impl std::ops::Index<usize> for VectorRegisters {
    type Output = Vreg;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vregs[index]
    }
}

impl std::ops::IndexMut<usize> for VectorRegisters {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vregs[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api() {
        let mut vregs = VectorRegisters::default();

        let result: Vreg = vregs.acquire2(0, 8).execute(|rs1_el, rs2_el| rs1_el + rs2_el);
        vregs[0] = result;
    }
}