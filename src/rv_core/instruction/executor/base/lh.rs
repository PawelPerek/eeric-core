use crate::rv_core::{instruction::format::I, memory::Memory, registers::IntegerRegisters};

pub fn lh(I { rd, rs1: rs, imm12 }: I, x: &mut IntegerRegisters, memory: &Memory) {
    let address: usize = (x[rs] + imm12) as usize;
    let value = u16::from_le_bytes(memory.0[address..address + 2].try_into().unwrap());
    x[rd] = value as u64;
}
