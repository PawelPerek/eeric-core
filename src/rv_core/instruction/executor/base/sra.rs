use crate::rv_core::{
    instruction::format::base::R, 
    registers::integer::IntegerRegisters
};

pub fn sra(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    let rs1 = x[rs1] as i64;
    let rs2 = x[rs2] as i64 & 0b11111;
    x[rd] = (rs1 >> rs2) as u64;
}