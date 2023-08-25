use crate::rv_core::{instruction::format::I, registers::IntegerRegisters};

pub fn ori(I { rd, rs1, imm12 }: I, x: &mut IntegerRegisters) {
    x[rd] = x[rs1] | imm12;
}
