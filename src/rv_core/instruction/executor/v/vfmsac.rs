use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(
    Opfvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opfvv,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
) {
    let vreg = izip!(
        v.get(vs2, vec_engine).iter_fp(),
        v.get(vs1, vec_engine).iter_fp(),
        v.get(vd, vec_engine).iter_fp()
    )
    .masked_map(
        v.default_mask(vm, vec_engine),
        v.get(vd, vec_engine).iter_fp(),
        |(vs2, vs1, vd)| (vs2 * vs1) - vd,
    )
    .collect_fp();

    v.apply(vd, vreg, vec_engine);
}

pub fn vf(
    Opfvf { vd, rs1, vs2, vm }: Opfvf,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
    f: &FloatRegisters,
) {
    let vreg = izip!(
        v.get(vs2, vec_engine).iter_fp(),
        v.get(vd, vec_engine).iter_fp()
    )
    .masked_map(
        v.default_mask(vm, vec_engine),
        v.get(vd, vec_engine).iter_fp(),
        |(vs2, vd)| (vs2 * ArbitraryFloat::copy_type(&vs2, f[rs1])) - vd,
    )
    .collect_fp();

    v.apply(vd, vreg, vec_engine);
}
