use crate::rv_core::{instruction::format::I, registers::IntegerRegisters};

pub fn andi(I { rd, rs1, imm12 }: I, x: &mut IntegerRegisters) {
    x[rd] = x[rs1] & imm12;
}
