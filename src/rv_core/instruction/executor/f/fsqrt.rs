use crate::rv_core::{instruction::format::R, registers::FloatRegisters};

use super::utils::convert::{compose, decompose};

pub fn s(R { rd, rs1, rs2: _ }: R, f: &FloatRegisters) {
    let (fs1, _) = decompose(f[rs1]);
    let (_, rest) = decompose(f[rd]);

    f[rd] = compose(fs1.sqrt(), rest);
}
