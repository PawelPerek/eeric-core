use crate::rv_core::{instruction::format::I, memory::Memory, registers::IntegerRegisters};

pub fn lw(I { rd, rs1: rs, imm12 }: I, x: &mut IntegerRegisters, memory: &Memory) {
    let address = (x[rs] + imm12) as usize;
    let value = u32::from_le_bytes(memory.0[address..address + 4].try_into().unwrap());
    x[rd] = value as u64;
}
