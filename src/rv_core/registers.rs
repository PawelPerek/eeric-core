pub mod integer;
pub mod float;
pub mod vector;

use integer::IntegerRegisters;
use float::FloatRegisters;
use vector::VectorRegisters;

#[derive(Clone, Default)]
pub struct Registers {
    pub x: IntegerRegisters,
    pub f: FloatRegisters,
    pub v: VectorRegisters,
    pub pc: u64
}

impl Registers {
    fn new_zeros() -> Registers {
        Registers {
            x: Default::default(),
            f: Default::default(),
            v: Default::default(),
            pc: 0
        }
    }
}
