use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{Opivi, Opivv, Opivx},
    registers::{IntegerRegisters, VectorRegisters},
};

pub fn vvm(
    Opivv {
        vd,
        vs1,
        vs2,
        vm: _,
    }: Opivv,
    v: &mut VectorRegisters,
) {
    let vreg = izip!(
        v.get(vs1).iter_eew(),
        v.get(vs2).iter_eew(),
        v.default_mask(true)
    )
    .map(|vel| vel.0 + vel.1 + vel.2)
    .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn vxm(
    Opivx {
        vd,
        rs1,
        vs2,
        vm: _,
    }: Opivx,
    v: &mut VectorRegisters,
    x: &IntegerRegisters,
) {
    let vreg = izip!(v.get(vs2).iter_eew(), v.default_mask(true))
        .map(|(vel, mask)| vel + x[rs1] + mask)
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn vim(
    Opivi {
        vd,
        imm5,
        vs2,
        vm: _,
    }: Opivi,
    v: &mut VectorRegisters,
) {
    let vreg = izip!(v.get(vs2).iter_eew(), v.default_mask(true))
        .map(|(vel, mask)| vel + imm5 + mask)
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
