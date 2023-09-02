use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vmunary0 {
        dest: vd,
        vs2: _,
        vm,
        ..
    }: Vmunary0,
    v: &mut VectorRegisters, vec_engine: &VectorEngine,
) {
    let vreg = v
        .get(vd, vec_engine)
        .iter_eew()
        .enumerate()
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |(i, _)| i as u64)
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
