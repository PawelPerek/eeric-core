use crate::rv_core::{instruction::format::R, registers::IntegerRegisters};

pub fn or(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    x[rd] = x[rs1] | x[rs2];
}
