use crate::rv_core::{instruction::format::I, memory::Memory, registers::IntegerRegisters};

pub fn lwu(I { rd, rs1: rs, imm12 }: I, x: &mut IntegerRegisters, memory: &Memory) {
    todo!();
}
