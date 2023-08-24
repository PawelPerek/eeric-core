use crate::rv_core::{instruction::format::I, memory::Memory, registers::IntegerRegisters};

pub fn lbu(I { rd, rs1: rs, imm12 }: I, x: &mut IntegerRegisters, memory: &Memory) {
    todo!();
}
