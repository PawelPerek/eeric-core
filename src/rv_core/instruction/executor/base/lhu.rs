use crate::rv_core::{instruction::format::I, memory::Memory, registers::IntegerRegisters};

pub fn lhu(I { rd, rs1: rs, imm12 }: I, x: &mut IntegerRegisters, memory: &Memory) {
    todo!();
}
