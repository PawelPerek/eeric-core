use crate::rv_core::{
    instruction::format::base::R, 
    registers::integer::IntegerRegisters
};

pub fn srl(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    let rs1 = x[rs1];
    let shamt5 = x[rs2] & 0b11111;
    x[rd] = rs1 >> rs2;
}