use crate::rv_core::{
    instruction::format::base::R, 
    registers::integer::IntegerRegisters
};

pub fn sltu(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    x[rd] = if x[rs1] < x[rs2] { 1 } else { 0 };
}