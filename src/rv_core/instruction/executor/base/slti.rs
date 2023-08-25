use crate::rv_core::{instruction::format::I, registers::IntegerRegisters};

pub fn slti(I { rd, rs1, imm12 }: I, x: &mut IntegerRegisters) {
    x[rd] = if x[rs1] < imm12 { 1 } else { 0 };
}
