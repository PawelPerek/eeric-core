use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters, vec_engine: &VectorEngine) {
    let vlmax = vec_engine.vlmax();

    let vs2_state = v.get(vs2, vec_engine).iter_eew().collect_vec();

    let vreg = v
        .get(vs1, vec_engine)
        .iter_eew_e16()
        .take(vlmax)
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get(vd, vec_engine).iter_eew(),
            |vindex| {
                if vindex as usize >= vlmax {
                    0
                } else {
                    vs2_state[vindex as usize]
                }
            },
        )
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
