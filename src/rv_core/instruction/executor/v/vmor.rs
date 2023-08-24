use crate::prelude::*;

use crate::rv_core::{instruction::format::Opmvv, registers::VectorRegisters};

pub fn mm(
    Opmvv {
        dest: vd,
        vs1,
        vs2,
        vm: _,
    }: Opmvv,
    v: &mut VectorRegisters,
) {
    let vreg = izip!(
        v.get(vd).iter_eew(),
        v.get(vs1).iter_mask(),
        v.get(vs2).iter_mask(),
    )
    .map(|(vel, m1, m2)| vel.with_mask_bit(m1 | m2))
    .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
