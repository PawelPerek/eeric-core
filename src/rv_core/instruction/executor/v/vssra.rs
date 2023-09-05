use crate::rv_core::instruction::executor::prelude::*;

use super::utils::{rounding::Roundoff, shamt::shamt};

pub fn vv(
    Opivv { vd, vs1, vs2, vm }: Opivv,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
    csr: &CsrRegisters,
) {
    let roundoff_signed = Roundoff::new_signed(csr);

    let vreg = izip!(
        v.get(vs2, vec_engine).iter_eew(),
        v.get(vs1, vec_engine).iter_eew()
    )
    .masked_map(
        v.default_mask(vm, vec_engine),
        v.get(vd, vec_engine).iter_eew(),
        |(vs2, vs1)| roundoff_signed(vs2 as u128, shamt(vs1, vec_engine.sew) as u8),
    )
    .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn vx(
    Opivx { vd, rs1, vs2, vm }: Opivx,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
    x: &IntegerRegisters,
    csr: &CsrRegisters,
) {
    let roundoff_signed = Roundoff::new_signed(csr);

    let vreg = v
        .get(vs2, vec_engine)
        .iter_eew()
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get(vd, vec_engine).iter_eew(),
            |vs2| roundoff_signed(vs2 as u128, shamt(x[rs1], vec_engine.sew) as u8),
        )
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn vi(
    Opivi { vd, imm5, vs2, vm }: Opivi,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
    csr: &CsrRegisters,
) {
    let roundoff_signed = Roundoff::new_signed(csr);

    let vreg = v
        .get(vs2, vec_engine)
        .iter_eew()
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get(vd, vec_engine).iter_eew(),
            |vs2| roundoff_signed(vs2 as u128, shamt(imm5 as u64, vec_engine.sew) as u8),
        )
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
