use crate::rv_core::{instruction::format::R, registers::IntegerRegisters};

pub fn sraw(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    let rs1 = x[rs1] as i32;
    let shamt = x[rs2] & 0b11111;
    x[rd] = (rs1 >> shamt) as u64;
}
