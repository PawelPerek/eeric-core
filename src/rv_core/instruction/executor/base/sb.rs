use crate::rv_core::{
    instruction::format::S, 
    registers::IntegerRegisters, 
    memory::Memory
};

pub fn sb(S { rs1, rs2, imm12 }: S, x: &IntegerRegisters, memory: &mut Memory) {
    todo!();
}