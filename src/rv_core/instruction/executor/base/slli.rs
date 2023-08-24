use crate::rv_core::{instruction::format::I, registers::IntegerRegisters};

pub fn slli(I { rd, rs1: rs, imm12 }: I, x: &mut IntegerRegisters) {
    let rs = x[rs];
    let shamt5 = imm12 & 0b11111;
    x[rd] = rs << shamt5;
}
