use crate::rv_core::instruction::executor::prelude::*;

pub fn vm(
    Opmvv {
        dest: vd,
        vs1,
        vs2,
        vm: _,
    }: Opmvv,
    v: &mut VectorRegisters, vec_engine: &VectorEngine,
) {
    let vreg = izip!(v.get(vs2, vec_engine).iter_eew(), v.get(vs1, vec_engine).iter_mask())
        .filter_map(|(vs2, vs1)| match vs1 {
            0 => None,
            _ => Some(vs2),
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
