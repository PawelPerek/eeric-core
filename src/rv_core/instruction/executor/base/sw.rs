use crate::rv_core::{
    instruction::format::base::S, 
    registers::integer::IntegerRegisters, 
    memory::Memory
};

pub fn sw(S { rs1, rs2, imm12 }: S, x: &IntegerRegisters, memory: &mut Memory) {
    todo!();
}