use crate::prelude::*;

use crate::rv_core::{
    instruction::format::Opmvv, 
    registers::VectorRegisters
};

pub fn vm(Opmvv { dest: vd, vs1, vs2, vm }: Opmvv, v: &mut VectorRegisters) {
    let vlmax = v.vec_engine.vlmax();

    let vreg = izip!(
        v.get(vs2).iter_eew(),
        v.get(vs1).iter_mask()
    ) 
        .filter_map(|(vel, mask)| match mask {
            0 => None,
            _ => Some(vel)
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.partial_apply(vd, vreg);
}