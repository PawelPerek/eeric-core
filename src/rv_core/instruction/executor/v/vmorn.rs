use crate::rv_core::instruction::executor::prelude::*;

pub fn mm(
    Opmvv {
        dest: vd,
        vs1,
        vs2,
        vm: _,
    }: Opmvv,
    v: &mut VectorRegisters, vec_engine: &VectorEngine,
) {
    let vreg = izip!(
        v.get(vd, vec_engine).iter_eew(),
        v.get(vs2, vec_engine).iter_mask(),
        v.get(vs1, vec_engine).iter_mask(),
    )
    .map(|(vd, vs2, vs1)| vd.with_mask_bit(vs2 | !vs1))
    .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
