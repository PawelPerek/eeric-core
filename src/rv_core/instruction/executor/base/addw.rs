use crate::rv_core::{instruction::format::R, registers::IntegerRegisters};

pub fn addw(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    x[rd] = (x[rs1] as i32 + x[rs2] as i32) as u64;
}
