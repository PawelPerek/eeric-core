use crate::rv_core::{
    instruction::format::base::I, 
    registers::integer::IntegerRegisters, 
    memory::Memory
};

pub fn lbu(I { rd, rs, imm12 }: I, x: &mut IntegerRegisters, memory: &Memory) {
    todo!();
}