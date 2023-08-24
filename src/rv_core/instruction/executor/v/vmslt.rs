use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{Opivv, Opivx},
    registers::{IntegerRegisters, VectorRegisters},
};

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters) {
    let vreg = izip!(v.get(vs2).iter_mask(), v.get(vs1).iter_mask(),)
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |(vs2, vs1)| {
            if (vs2 as i64) < (vs1 as i64) {
                1
            } else {
                0
            }
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    let vreg = v
        .get(vs2)
        .iter_mask()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            if (vs2 as i64) < (x[rs1] as i64) {
                1
            } else {
                0
            }
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
