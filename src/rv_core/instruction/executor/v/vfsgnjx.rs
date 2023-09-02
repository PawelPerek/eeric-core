use num_traits::Float;

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
        v.get(vs1, vec_engine).iter_fp()
    )
    .masked_map(
        v.default_mask(vm, vec_engine),
        v.get(vd, vec_engine).iter_fp(),
        |(vs2, vs1)| match (vs2.is_sign_positive(), vs1.is_sign_positive()) {
            (true, true) => vs2,
            (true, false) => -vs2,
            (false, true) => -vs2,
            (false, false) => vs2,
        },
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
    let vreg = v
        .get(vs2, vec_engine)
        .iter_fp()
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get(vd, vec_engine).iter_fp(),
            |vs2| match (vs2.is_sign_positive(), f[rs1].is_sign_positive()) {
                (true, true) => vs2,
                (true, false) => -vs2,
                (false, true) => -vs2,
                (false, false) => vs2,
            },
        )
        .collect_fp();

    v.apply(vd, vreg, vec_engine);
}
