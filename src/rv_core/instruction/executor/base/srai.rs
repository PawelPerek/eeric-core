use crate::rv_core::{
    instruction::format::I, 
    registers::integer::IntegerRegisters
};

pub fn srai(I { rd, rs1: rs, imm12 }: I, x: &mut IntegerRegisters) {
    let rs = x[rs] as i64;
    let shamt = imm12 & 0b11111;
    x[rd] = (rs >> shamt) as u64;
}