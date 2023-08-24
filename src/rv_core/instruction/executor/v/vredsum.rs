use crate::prelude::*;

use crate::rv_core::{instruction::format::Opmvv, registers::VectorRegisters};

pub fn vs(
    Opmvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opmvv,
    v: &mut VectorRegisters,
) {
    let initial_value = v.get(vs1).iter_eew().next().unwrap();
    let sum = izip!(v.get(vs2).iter_eew(), v.default_mask(vm))
        .fold(initial_value, |acc, (vs2, mask)| {
            acc.wrapping_add(vs2 * mask)
        });

    let mut vd_data = v.get(vd).iter_eew().collect_vec();
    vd_data[0] = sum;

    let vreg = vd_data
        .into_iter()
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
