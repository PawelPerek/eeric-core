use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{
        Opivv,
        Opivx,
        Opivi,
    }, 
    registers::{
        VectorRegisters, 
        IntegerRegisters
    }
};

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters) {
    let vreg = izip!(
        v.get(vs1).iter_eew(),
        v.get(vs2).iter_eew()
    ).masked_map(
        v.default_mask(vm),
        v.get(vd).iter_eew(),
        |vel| vel.0 | vel.1
    ).collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    let vreg = 
        v.get(vs2).iter_eew()
        .masked_map(
            v.default_mask(vm),
            v.get(vd).iter_eew(),
            |vel| vel | x[rs1]
        ).collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn vi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters) {
    let vreg = 
        v.get(vs2).iter_eew()
        .masked_map(
            v.default_mask(vm),
            v.get(vd).iter_eew(),
            |vel| vel | imm5
        ).collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
