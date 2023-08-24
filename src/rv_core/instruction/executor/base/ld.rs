use crate::rv_core::{instruction::format::I, memory::Memory, registers::IntegerRegisters};

pub fn ld(I { rd, rs1: rs, imm12 }: I, x: &mut IntegerRegisters, memory: &Memory) {
    let address = (x[rs] + imm12) as usize;
    let value = u64::from_le_bytes(memory.0[address..address + 8].try_into().unwrap());
    x[rd] = value;
}
