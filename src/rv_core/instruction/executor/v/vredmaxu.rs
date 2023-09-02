use crate::rv_core::instruction::executor::prelude::*;

pub fn vs(
    Opmvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opmvv,
    v: &mut VectorRegisters, vec_engine: &VectorEngine,
) {
    let initial_value = v.get(vs1, vec_engine).iter_eew().next().unwrap();
    let sum = izip!(v.get(vs2, vec_engine).iter_eew(), v.default_mask(vm, vec_engine)).fold(
        initial_value,
        |max_val, (vs2, mask)| {
            if mask == 1 && vs2 > max_val {
                vs2
            } else {
                max_val
            }
        },
    );

    let mut vd_data = v.get(vd, vec_engine).iter_eew().collect_vec();
    vd_data[0] = sum;

    let vreg = vd_data
        .into_iter()
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
