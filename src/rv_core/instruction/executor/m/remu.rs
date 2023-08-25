use crate::rv_core::{instruction::format::R, registers::IntegerRegisters};

pub fn remu(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    x[rd] = if x[rs2] == 0 {
        x[rs1]
    } else {
        x[rs1].wrapping_rem(x[rs2])
    }
}
