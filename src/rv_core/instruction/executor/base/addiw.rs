use crate::rv_core::{instruction::format::I, registers::IntegerRegisters};

pub fn addiw(I { rd, rs1, imm12 }: I, x: &mut IntegerRegisters) {
    x[rd] = (x[rs1] as i32 + imm12 as i32) as u64;
}
