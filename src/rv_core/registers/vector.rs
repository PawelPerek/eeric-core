mod vector_engine;
mod vreg;

use vector_engine::VectorEngine;
use vreg::Vreg;

use self::vector_engine::SEW;

#[derive(Clone)]
pub struct VectorRegisters{
    // Vector register holds 32 * VLENB bytes
    raw: Vec<u8>,
    vec_engine: VectorEngine
}

impl VectorRegisters {
    fn start_ptr(&self, nth: usize) -> usize {
        nth * self.vec_engine.vlenb()
    }

    pub fn register_view(&self, nth: usize) -> Vec<u8> {
        let start = self.start_ptr(nth);

        // Note: Since we are working on multiples of two
        // multiplying 2^n (vlenb) by 2^(-n) (lmul) will not create floating point errors
        let end = start + (self.vec_engine.vlenb() as f32 * self.vec_engine.lmul()) as usize - 1;

        let range = start .. end;

        self.raw[range].to_owned()
    }

    pub fn acquire(&self, rs1: usize) -> AcquiredRegister {
        AcquiredRegister { 
            rs: Vreg::new(
                self.register_view(rs1), 
                SEW::new(self.vec_engine.sew()).unwrap()
            )
        }
    }

    pub fn acquire2<'a>(&'a self, rs1: usize, rs2: usize ) -> Acquired2Registers {
        Acquired2Registers { 
            rs1: Vreg::new(
                self.register_view(rs1), 
                SEW::new(self.vec_engine.sew()).unwrap()
            ), 
            rs2: Vreg::new(
                self.register_view(rs2), 
                SEW::new(self.vec_engine.sew()).unwrap()
            )
        }
    }

    pub fn get(&self, nth: usize) -> Vreg {
        Vreg::new(self.register_view(nth), SEW::new(self.vec_engine.sew()).unwrap())
    }

    pub fn apply(&mut self, nth: usize, vreg: Vreg) {
        let start = self.start_ptr(nth);
        self.raw[start..].copy_from_slice(&vreg.raw)
    }
}

pub struct AcquiredRegister{
    rs: Vreg
}

pub struct Acquired2Registers{
    rs1: Vreg,
    rs2: Vreg
}


impl AcquiredRegister {
    pub fn execute<F>(&self, builder: F) -> Vreg 
        where
            F: Fn(u64) -> u64 
    {
        // TODO: sew pass can be probably done more elegantly
        Vreg::new(
            self.rs.clone().map(builder).flat_map(u64::to_le_bytes).collect(),
            self.rs.sew.clone()
        )
    }
}


impl Acquired2Registers {
    pub fn execute<F>(&self, builder: F) -> Vreg 
        where
            F: Fn((u64, u64)) -> u64 
    {
        Vreg::new(
            self.rs1.clone().zip(self.rs2.clone()).map(builder).flat_map(u64::to_le_bytes).collect(), 
            self.rs1.sew.clone()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api() {
        let mut vregs = VectorRegisters {
            raw: Default::default(),
            vec_engine: Default::default()
        };

        let result: Vreg = vregs.acquire2(0, 8).execute(|(rs1_el, rs2_el)| rs1_el + rs2_el);
        vregs.apply(0, result);
    }
}