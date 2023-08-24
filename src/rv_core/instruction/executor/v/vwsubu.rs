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
    let vreg = izip!(v.get(vs1).iter_eew(), v.get(vs2).iter_eew())
        .masked_map(
            v.default_mask(vm),
            v.get_wide(vd).iter_eew(),
            |(vs1, vs2)| (vs2 as u128) - (vs1 as u128),
        )
        .collect_with_wide_eew(v.vec_engine.sew.clone());

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
        .masked_map(v.default_mask(vm), v.get_wide(vd).iter_eew(), |vs2| {
            (vs2 as u128) - (x[rs1] as u128)
        })
        .collect_with_wide_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn wv(
    Opmvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opmvv,
    v: &mut VectorRegisters,
) {
    let vreg = izip!(v.get(vs1).iter_eew(), v.get_wide(vs2).iter_eew())
        .masked_map(
            v.default_mask(vm),
            v.get_wide(vd).iter_eew(),
            |(vs1, vs2)| vs2 - (vs1 as u128),
        )
        .collect_with_wide_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn wx(Opmvx { dest: vd, rs1, vs2, vm }: Opmvx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    let vreg = v
        .get_wide(vs2)
        .iter_eew()
        .masked_map(
            v.default_mask(vm),
            v.get_wide(vd).iter_eew(),
            |vs2| vs2 - (x[rs1] as u128),
        )
        .collect_with_wide_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
