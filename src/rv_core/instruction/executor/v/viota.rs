use crate::prelude::*;

use crate::rv_core::{instruction::format::Vmunary0, registers::VectorRegisters};

pub fn m(
    Vmunary0 {
        dest: vd, vs2, vm, ..
    }: Vmunary0,
    v: &mut VectorRegisters,
) {
    let mut sum = 0u64;

    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            let sum_snapshot = sum;
            if vs2 != 0 {
                sum += 1;
            }
            sum_snapshot
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
