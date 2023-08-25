use crate::rv_core::{instruction::format::R, registers::FloatRegisters};

pub fn d(R { rd, rs1, rs2 }: R, f: &mut FloatRegisters) {
    let (fs1_sign, fs2_sign) = (
        if f[rs1].is_sign_positive() { 0 } else { 1 },
        if f[rs2].is_sign_positive() { 0 } else { 1 }
    );

    let result_sign = fs1_sign ^ fs2_sign;

    f[rd] = if result_sign == 0 { f[rs1] } else { -f[rs1] };
}
