use crate::rv_core::instruction::executor::prelude::*;

pub fn vvm(
    Opivv {
        vd,
        vs1,
        vs2,
        vm: _,
    }: Opivv,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
) {
    let vreg = izip!(
        v.default_mask(true, vec_engine),
        v.get(vs2, vec_engine).iter_eew(),
        v.get(vs1, vec_engine).iter_eew(),
    )
    .map(|(mask, vs2, vs1)| if mask == 1 { vs1 } else { vs2 })
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
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
    x: &IntegerRegisters,
) {
    let vreg = izip!(
        v.default_mask(true, vec_engine),
        v.get(vs2, vec_engine).iter_eew(),
    )
    .map(|(mask, vs2)| if mask == 1 { x[rs1] } else { vs2 })
    .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn vim(
    Opivi {
        vd,
        imm5,
        vs2,
        vm: _,
    }: Opivi,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
) {
    let vreg = izip!(
        v.default_mask(true, vec_engine),
        v.get(vs2, vec_engine).iter_eew(),
    )
    .map(|(mask, vs2)| if mask == 1 { imm5 as u64 } else { vs2 })
    .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
