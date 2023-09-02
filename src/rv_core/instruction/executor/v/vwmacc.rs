use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(
    Opmvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opmvv,
    v: &mut VectorRegisters, vec_engine: &VectorEngine,
) {
    let vreg = izip!(
        v.get(vs1, vec_engine).iter_eew(),
        v.get(vs2, vec_engine).iter_eew(),
        v.get_wide(vd, vec_engine).iter_eew()
    )
    .masked_map(
        v.default_mask(vm, vec_engine),
        v.get_wide(vd, vec_engine).iter_eew(),
        |(vs1, vs2, vd)| {
            (vs2 as i64 as u128)
                .wrapping_mul(vs1 as i64 as u128)
                .wrapping_add(vd)
        },
    )
    .collect_with_wide_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn vx(
    Opmvx {
        dest: vd,
        rs1,
        vs2,
        vm,
    }: Opmvx,
    v: &mut VectorRegisters, vec_engine: &VectorEngine,
    x: &IntegerRegisters,
) {
    let vreg = izip!(v.get(vs2, vec_engine).iter_eew(), v.get_wide(vd, vec_engine).iter_eew())
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get_wide(vd, vec_engine).iter_eew(),
            |(vs2, vd)| {
                (vs2 as i64 as u128)
                    .wrapping_mul(x[rs1] as i64 as u128)
                    .wrapping_add(vd)
            },
        )
        .collect_with_wide_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
