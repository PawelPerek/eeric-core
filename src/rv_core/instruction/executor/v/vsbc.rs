use crate::rv_core::instruction::executor::prelude::*;

pub fn vvm(
    Opivv {
        vd,
        vs1,
        vs2,
        vm: _,
    }: Opivv,
    v: &mut VectorRegisters, vec_engine: &VectorEngine,
) {
    let vreg = izip!(
        v.get(vs2, vec_engine).iter_eew(),
        v.get(vs1, vec_engine).iter_eew(),
        v.default_mask(true, vec_engine)
    )
    .map(|(vs2, vs1, mask)| vs2.wrapping_sub(vs1).wrapping_sub(mask))
    .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn vxm(
    Opivx {
        vd,
        rs1,
        vs2,
        vm: _,
    }: Opivx,
    v: &mut VectorRegisters, vec_engine: &VectorEngine,
    x: &IntegerRegisters,
) {
    let vreg = izip!(v.get(vs2, vec_engine).iter_eew(), v.default_mask(true, vec_engine))
        .map(|(vs2, mask)| vs2.wrapping_sub(x[rs1]).wrapping_sub(mask))
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
