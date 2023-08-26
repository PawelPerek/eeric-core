use crate::prelude::*;

use crate::rv_core::{instruction::format::R, registers::FloatRegisters};


pub fn s(R { rd, rs1, rs2 }: R, f: &mut FloatRegisters) {
    let (fs1, _) = decompose(f[rs1]);
    let (fs2, _) = decompose(f[rs2]);

    let (_, rest) = decompose(f[rd]);

    let (fs1_sign, fs2_sign) = (
        if fs1.is_sign_positive() { 0 } else { 1 },
        if fs2.is_sign_positive() { 0 } else { 1 }
    );

    let result_sign = fs1_sign ^ fs2_sign;

    f[rd] = compose(
        if result_sign == 0 { fs1 } else { -fs1 },
        rest
    );
}