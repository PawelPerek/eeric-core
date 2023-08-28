pub mod aliases;
mod csr;
mod float;
mod integer;
pub mod vector;

pub use csr::CsrRegisters;
pub use float::FloatRegisters;
pub use integer::IntegerRegisters;
pub use vector::VectorRegisters;

#[derive(Clone, Default)]
pub struct Registers {
    pub x: IntegerRegisters,
    pub c: CsrRegisters,
    pub f: FloatRegisters,
    pub v: VectorRegisters,
    pub pc: u64,
}

#[derive(Debug)]
pub struct RegistersSnapshot {
    pub x: [u64; 31],
    pub c: [u64; 4096],
    pub f: [f64; 32],
    pub v: [Vec<u8>; 32],
    pub pc: u64,
}


impl Registers {
    pub fn snapshot(&self) -> RegistersSnapshot {
        RegistersSnapshot {
            x: self.x.snapshot(),
            c: self.c.snapshot(),
            f: self.f.snapshot(),
            v: self.v.snapshot(),
            pc: self.pc,
        }
    }
}