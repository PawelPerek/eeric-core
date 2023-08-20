use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{Opmvv, Opmvx}, 
    registers::{VectorRegisters, IntegerRegisters}
};

pub fn vv(Opmvv { dest: vd, vs1, vs2, vm }: Opmvv, v: &mut VectorRegisters) {
    let vreg = izip!(
        v.get(vd).iter_eew(),
        v.get(vs1).iter_eew(),
        v.get(vs2).iter_eew()
    ).masked_map(
        v.default_mask(vm),
        v.get(vd).iter_eew(),
        |(vd, vs1, vs2)| vd + (vs1 * vs2)
    ).collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn vx(Opmvx { dest: vd, rs1, vs2, vm }: Opmvx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    let vreg = izip!(
        v.get(vd).iter_eew(),
        v.get(vs2).iter_eew()
    ).masked_map(
        v.default_mask(vm),
        v.get(vd).iter_eew(),
        |(vd, vs2)| vd + (x[rs1] * vs2)
    ).collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
