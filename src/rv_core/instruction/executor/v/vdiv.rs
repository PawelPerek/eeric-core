use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{Opmvv, Opmvx},
    registers::{IntegerRegisters, VectorRegisters},
};

pub fn vv(
    Opmvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opmvv,
    v: &mut VectorRegisters,
) {
    let vreg = izip!(v.get(vs2).iter_eew(), v.get(vs1).iter_eew())
        .masked_map(
            v.default_mask(vm),
            v.get(vd).iter_eew(),
            |(dividend, divisor)| {
                if divisor == 0 {
                    u64::MAX
                } else {
                    ((dividend as i64) / (divisor as i64)) as u64
                }
            },
        )
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn vx(
    Opmvx {
        dest: vd,
        rs1,
        vs2,
        vm,
    }: Opmvx,
    v: &mut VectorRegisters,
    x: &IntegerRegisters,
) {
    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |dividend| {
            let divisor = x[rs1];

            if divisor == 0 {
                u64::MAX
            } else {
                ((dividend as i64) / (divisor as i64)) as u64
            }
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
