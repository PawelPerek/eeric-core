use crate::rv_core::instruction::executor::prelude::*;

pub fn vs(
    Opfvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opfvv,
    v: &mut VectorRegisters, vec_engine: &VectorEngine,
) {
    let initial_value = v.get(vs1, vec_engine).iter_fp().next().unwrap();
    let sum =
        izip!(v.get(vs2, vec_engine).iter_fp(), v.default_mask(vm, vec_engine)).fold(initial_value, |acc, (vs2, mask)| {
            if mask == 1 {
                acc + vs2
            } else {
                acc
            }
        });

    let mut vd_snapshot = v.get(vd, vec_engine).iter_fp().collect_vec();
    vd_snapshot[0] = sum;

    let vreg = vd_snapshot.into_iter().collect_fp();

    v.apply(vd, vreg, vec_engine);
}
