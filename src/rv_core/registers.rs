mod csr;
mod float;
mod integer;
mod vector;

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
    pub pc: u64
}

impl Registers {
    fn new_zeros() -> Registers {
        Registers {
            x: Default::default(),
            c: Default::default(),
            f: Default::default(),
            v: Default::default(),
            pc: 0
        }
    }
}
