use crate::prelude::*;

use crate::rv_core::{instruction::format::R, registers::FloatRegisters};

pub fn s(R { rd, rs1, rs2 }: R, f: &mut FloatRegisters) {
    let (fs1, _) = decompose(f[rs1]);
    let (fs2, _) = decompose(f[rs2]);

    let (_, rest) = decompose(f[rd]);

    f[rd] = compose(fs1.copysign(fs2), rest);
}
