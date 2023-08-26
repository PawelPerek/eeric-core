use crate::prelude::*;

use crate::rv_core::{instruction::format::R4, registers::FloatRegisters};


pub fn s(R4 { rd, rs1, rs2, rs3 }: R4, f: &mut FloatRegisters) {
    let (fs1, _) = decompose(f[rs1]);
    let (fs2, _) = decompose(f[rs2]);
    let (fs3, _) = decompose(f[rs3]);
    let (_, rest) = decompose(f[rd]);

    f[rd] = compose(fs1 * fs2 + fs3, rest);
}
