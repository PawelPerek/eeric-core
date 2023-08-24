use crate::rv_core::{instruction::format::I, memory::Memory, registers::IntegerRegisters};

pub fn lb(I { rd, rs1: rs, imm12 }: I, x: &mut IntegerRegisters, memory: &Memory) {
    let address: usize = (x[rs] + imm12) as usize;
    let value = memory.0[address];
    x[rd] = value as u64;
}
