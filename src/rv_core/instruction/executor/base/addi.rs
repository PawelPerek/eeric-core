use crate::rv_core::{
    instruction::format::I, 
    registers::integer::IntegerRegisters
};

pub fn addi(I { rd, rs1: rs, imm12 }: I, x: &mut IntegerRegisters) {
    x[rd] = x[rs] + imm12;
}