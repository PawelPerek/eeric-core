use crate::rv_core::{
    instruction::format::R, 
    registers::integer::IntegerRegisters
};

pub fn add(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    x[rd] = x[rs1] + x[rs2];
}