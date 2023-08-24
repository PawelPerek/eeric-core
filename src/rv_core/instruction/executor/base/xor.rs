use crate::rv_core::{instruction::format::R, registers::IntegerRegisters};

pub fn xor(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    x[rd] = x[rs1] ^ x[rs2];
}
