use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vfunary1 {
        dest: vd, vs2, vm, ..
    }: Vfunary1,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
) {
    let vreg = v
        .get(vs2, vec_engine)
        .iter_fp()
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get(vd, vec_engine).iter_fp(),
            |vs2| ArbitraryFloat::copy_type(&vs2, 1.0) / vs2,
        )
        .collect_fp();

    v.apply(vd, vreg, vec_engine);
}
