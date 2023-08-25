use crate::rv_core::{instruction::format::S, memory::Memory, registers::IntegerRegisters};

pub fn sb(S { rs1, rs2, imm12 }: S, x: &IntegerRegisters, mem: &mut Memory) {
    let addr = x[rs1] + imm12;
    let bytes = (x[rs2] as u8).to_le_bytes();

    mem.set(addr as usize, bytes);
}
