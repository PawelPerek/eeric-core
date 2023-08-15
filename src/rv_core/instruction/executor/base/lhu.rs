use crate::rv_core::{
    instruction::format::I, 
    registers::IntegerRegisters, 
    memory::Memory
};

pub fn lhu(I { rd, rs1: rs, imm12 }: I, x: &mut IntegerRegisters, memory: &Memory) {
    todo!();
}