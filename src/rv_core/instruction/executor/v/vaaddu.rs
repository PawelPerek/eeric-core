use crate::rv_core::instruction::executor::prelude::*;

use super::utils::rounding::Roundoff;

pub fn vv(
    Opmvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opmvv,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
    csr: &CsrRegisters,
) {
    let roundoff_unsigned = Roundoff::new_unsigned(csr);

    let vreg = izip!(
        v.get(vs2, vec_engine).iter_eew(),
        v.get(vs1, vec_engine).iter_eew()
    )
    .masked_map(
        v.default_mask(vm, vec_engine),
        v.get(vd, vec_engine).iter_eew(),
        |(vs2, vs1)| roundoff_unsigned((vs2 as u128).wrapping_add(vs1 as u128), 1),
    )
    .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn vx(
    Opmvx {
        dest: vd,
        rs1,
        vs2,
        vm,
    }: Opmvx,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
    x: &IntegerRegisters,
    csr: &CsrRegisters,
) {
    let roundoff_unsigned = Roundoff::new_unsigned(csr);

    let vreg = v
        .get(vs2, vec_engine)
        .iter_eew()
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get(vd, vec_engine).iter_eew(),
            |vs2| roundoff_unsigned((vs2 as u128).wrapping_add(x[rs1] as u128), 1),
        )
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
