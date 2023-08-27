

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
    csr: &CsrRegisters,
) {
    let roundoff_signed = Roundoff::new_signed(csr);

    let vreg = izip!(v.get(vs2).iter_eew(), v.get(vs1).iter_eew())
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |(vs2, vs1)| {
            roundoff_signed(vs2 as u128 + vs1 as u128, 1)
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn vx(
    Opmvx { dest, rs1, vs2, vm }: Opmvx,
    v: &mut VectorRegisters,
    x: &IntegerRegisters,
    csr: &CsrRegisters,
) {
    let roundoff_signed = Roundoff::new_signed(csr);

    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(dest).iter_eew(), |vs2| {
            roundoff_signed(vs2 as u128 + x[rs1] as u128, 1)
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(dest, vreg);
}
