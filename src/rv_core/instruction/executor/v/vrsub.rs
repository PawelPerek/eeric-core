use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{Opivi, Opivx},
    registers::{IntegerRegisters, VectorRegisters},
};

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| x[rs1] - vs2)
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn vi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters) {
    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| imm5 - vs2)
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
