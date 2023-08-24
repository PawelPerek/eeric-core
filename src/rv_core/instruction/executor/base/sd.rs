use crate::rv_core::{instruction::format::S, memory::Memory, registers::IntegerRegisters};

pub fn sd(S { rs1, rs2, imm12 }: S, x: &IntegerRegisters, memory: &mut Memory) {
    todo!();
}
