use crate::rv_core::{
    instruction::format::I, 
    registers::IntegerRegisters
};

pub fn xori(I { rd, rs1: rs, imm12 }: I, x: &mut IntegerRegisters) {
    x[rd] = x[rs] ^ imm12;
}