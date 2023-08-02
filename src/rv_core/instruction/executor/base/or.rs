use crate::rv_core::{
    instruction::format::base::R, 
    registers::integer::IntegerRegisters
};

pub fn or(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    x[rd] = x[rs1] | x[rs2];
}