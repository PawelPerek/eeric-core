use crate::rv_core::{instruction::format::S, registers::{FloatRegisters, IntegerRegisters}, memory::Memory};

pub fn fsd(S { rs1, rs2, imm12 }: S, x: &IntegerRegisters, f: &FloatRegisters, mem: &mut Memory) {
    let addr = x[rs1] + imm12;
    let bytes = f[rs2].to_le_bytes();

    mem.set(addr as usize, bytes);
}
