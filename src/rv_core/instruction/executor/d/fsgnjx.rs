use crate::rv_core::instruction::executor::prelude::*;

pub fn d(R { rd, rs1, rs2 }: R, f: &mut FloatRegisters) {
    let value = match (f[rs1].is_sign_positive(), f[rs2].is_sign_positive()) {
        (true, true) => f[rs1],   // +x, +y => +x
        (true, false) => -f[rs1], // +x, -y => -x
        (false, true) => -f[rs1], // -x, +y => -x
        (false, false) => f[rs1], // -x, -y => +x
    };

    f[rd] = value;
}
