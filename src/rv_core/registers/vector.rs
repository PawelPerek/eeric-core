mod vector_engine;
mod vreg;

use std::{cell::RefCell, rc::Rc};

use vector_engine::VectorEngine;
use vreg::Vreg;

use self::vector_engine::SEW;

#[derive(Clone)]
pub struct VectorRegisters{
    // vector register holds 32 * VLENB bytes
    raw: Vec<u8>,
    vec_engine: VectorEngine
}

impl VectorRegisters {
    pub fn acquire<'a>(&'a self, rs1: usize) -> AcquiredRegisters {
        AcquiredRegister { rs: &self.vregs[rs1] }
    }

    pub fn acquire2<'a>(&'a self, rs1: usize, rs2: usize ) -> AcquiredRegisters {
        Acquired2Registers { rs1: &self.vregs[rs1], rs2: Some(&self.vregs[rs1]) }
    }
}

pub struct AcquiredRegister<'a>{
    rs: Vreg<'a>,
    sew: Rc<RefCell<SEW>>
}

pub struct Acquired2Registers<'a>{
    rs1: Vreg<'a>,
    rs2: Vreg<'a>,
    sew: Rc<RefCell<SEW>>
}


impl<'a> AcquiredRegister<'a> {
    fn execute<F>(&self, builder: F) -> Vreg 
        where
            F: Fn(u64) -> u64 
    {
        Vreg::new(
            self.rs.raw.iter().map(builder), 
            self.sew.clone()
        )
    }
}


impl<'a> Acquired2Registers<'a> {
    fn execute<F>(&self, builder: F) -> Vreg 
        where
            F: Fn(u64, u64) -> u64 
    {
        Vreg::new(
            self.rs1.raw.iter().zip(self.rs2.raw.iter()).map(builder), 
            self.sew.clone()
        )
    }
}

impl VectorRegisters {
    fn new(vlen: usize) -> Self {
        Self { 
            raw: vec![0x00; vlen * 32], 
            vec_engine: VectorEngine::new(
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default()
            ).unwrap()
        }
    }

    fn new_vlen128() -> Self { Self::new(128) }
}

impl Default for VectorRegisters {
    fn default() -> Self {
        Self::new_vlen128()
    }
}

impl std::ops::Index<usize> for VectorRegisters {
    type Output = Vreg<'a>;

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