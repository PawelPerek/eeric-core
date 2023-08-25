use crate::rv_core::{instruction::format::R, registers::FloatRegisters};

pub fn d(R { rd, rs1, rs2: _ }: R, f: &mut FloatRegisters) {
    f[rd] = f[rs1].sqrt();
}
