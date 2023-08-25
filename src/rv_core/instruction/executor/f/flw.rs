use crate::rv_core::{instruction::format::I, registers::FloatRegisters};

use super::utils::convert::{compose, decompose};

pub fn flw(I { rd, rs1: rs, imm12 }: I, f: &FloatRegisters) {
    todo!()
}
