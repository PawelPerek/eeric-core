use crate::rv_core::{instruction::format::R4, registers::FloatRegisters};

pub fn d(R4 { rd, rs1, rs2, rs3 }: R4, f: &mut FloatRegisters) {
    f[rd] = -f[rs1] * f[rs2] + f[rs3];
}
