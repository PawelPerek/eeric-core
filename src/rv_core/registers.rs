mod csr;
mod float;
mod integer;
pub mod vector;
pub mod aliases;

pub use csr::CsrRegisters;
pub use float::FloatRegisters;
pub use integer::IntegerRegisters;
pub use vector::VectorRegisters;
pub use vector::ArbitraryFloat;

#[derive(Clone, Default)]
pub struct Registers {
    pub x: IntegerRegisters,
    pub c: CsrRegisters,
    pub f: FloatRegisters,
    pub v: VectorRegisters,
    pub pc: u64,
}