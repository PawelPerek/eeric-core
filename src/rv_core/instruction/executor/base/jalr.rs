use crate::rv_core::{
    instruction::format::I, 
    registers::integer::IntegerRegisters
};

pub fn jalr(I { rd, rs1: rs, imm12 }: I, x: &IntegerRegisters) {
    todo!()
}