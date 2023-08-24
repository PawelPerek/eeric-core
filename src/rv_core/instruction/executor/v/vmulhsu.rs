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
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |(vs2, vs1)| {
            ((vs2 as i64 as u128) * (vs1 as u128) >> v.vec_engine.sew.bit_length() as u128) as u64
        })
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
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            ((vs2 as i64 as u128) * (x[rs1] as u128) >> v.vec_engine.sew.bit_length() as u128) as u64
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
