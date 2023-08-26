use crate::prelude::*;

use crate::rv_core::{instruction::format::I, registers::{FloatRegisters, IntegerRegisters}, memory::Memory};

pub fn flw(I { rd, rs1, imm12 }: I, x: &IntegerRegisters, f: &mut FloatRegisters, mem: &Memory) {
    let addr = x[rs1] + imm12;
    let fp = f32::from_le_bytes(mem.get(addr as usize));

    let (_, rest) = decompose(f[rd]);

    f[rd] = compose(fp, rest);
}
