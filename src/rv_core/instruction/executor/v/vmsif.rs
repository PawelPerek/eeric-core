use crate::prelude::*;

use crate::rv_core::{instruction::format::Vmunary0, registers::VectorRegisters};

pub fn m(
    Vmunary0 {
        dest: vd, vs2, vm, ..
    }: Vmunary0,
    v: &mut VectorRegisters,
) {
    let mut found_mask_bit = false;

    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            let ret = if found_mask_bit { 0 } else { 1 };

            if vs2 != 0 {
                found_mask_bit = true;
            }

            ret
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
