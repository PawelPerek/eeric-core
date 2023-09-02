use crate::rv_core::instruction::executor::prelude::*;

pub fn vs(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters, vec_engine: &VectorEngine) {
    let initial_value = v.get_wide(vs1, vec_engine).iter_eew().next().unwrap();
    let sum = izip!(
        v.get(vs2, vec_engine).iter_eew(),
        v.default_mask(vm, vec_engine)
    )
    .fold(initial_value, |acc, (vs2, mask)| {
        acc.wrapping_add(vs2 as i64 as u128 * mask as u128)
    });

    let mut vd_data = v.get_wide(vd, vec_engine).iter_eew().collect_vec();
    vd_data[0] = sum;

    let vreg = vd_data
        .into_iter()
        .collect_with_wide_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
