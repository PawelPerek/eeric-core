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
        v.default_mask(true),
        v.get(vs2).iter_eew(),
        v.get(vs1).iter_eew(),
    )
    .map(|(mask, vs2, vs1)| if mask == 1 { vs1 } else { vs2 })
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
    let vreg = izip!(v.default_mask(true), v.get(vs2).iter_eew(),)
        .map(|(mask, vs2)| if mask == 1 { x[rs1] } else { vs2 })
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
    let vreg = izip!(v.default_mask(true), v.get(vs2).iter_eew(),)
        .map(|(mask, vs2)| if mask == 1 { imm5 } else { vs2 })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
