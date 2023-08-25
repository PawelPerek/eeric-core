use crate::rv_core::{instruction::format::R, registers::IntegerRegisters};

pub fn divu(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    x[rd] = if x[rs2] == 0 {
        u64::MAX
    } else {
        x[rs1].wrapping_div(x[rs2])
    }
}
