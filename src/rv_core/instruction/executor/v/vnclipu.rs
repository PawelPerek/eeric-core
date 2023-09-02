use crate::rv_core::instruction::executor::prelude::*;

use super::utils::rounding::Roundoff;

pub fn wv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters, vec_engine: &VectorEngine, csr: &mut CsrRegisters) {
    let roundoff_signed = Roundoff::new_signed(csr);

    let uint_max = u64::MAX >> (64 - vec_engine.sew.bit_length());
    let sign_mask = u64::MIN << vec_engine.sew.bit_length();

    let vreg = izip!(v.get(vs2, vec_engine).iter_eew(), v.get(vs1, vec_engine).iter_eew())
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |(vs2, vs1)| {
            let result = roundoff_signed(
                vs2 as u128,
                vs1 as u8 & (2 * vec_engine.sew.bit_length() as u8 - 1),
            );

            if sign_mask & result != 0 {
                csr[VXSAT] = 1;
                uint_max as u64
            } else {
                result
            }
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn wx(
    Opivx { vd, rs1, vs2, vm }: Opivx,
    v: &mut VectorRegisters, vec_engine: &VectorEngine,
    x: &IntegerRegisters,
    csr: &mut CsrRegisters,
) {
    let roundoff_signed = Roundoff::new_signed(csr);

    let uint_max = u64::MAX >> (64 - vec_engine.sew.bit_length());
    let sign_mask = u64::MIN << vec_engine.sew.bit_length();

    let vreg = v
        .get(vs2, vec_engine)
        .iter_eew()
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |vs2| {
            let result = roundoff_signed(
                vs2 as u128,
                x[rs1] as u8 & (2 * vec_engine.sew.bit_length() as u8 - 1),
            );

            if sign_mask & result != 0 {
                csr[VXSAT] = 1;
                uint_max as u64
            } else {
                result
            }
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn wi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters, vec_engine: &VectorEngine, csr: &mut CsrRegisters) {
    let roundoff_signed = Roundoff::new_signed(csr);

    let uint_max = u64::MAX >> (64 - vec_engine.sew.bit_length());
    let sign_mask = u64::MIN << vec_engine.sew.bit_length();

    let vreg = v
        .get(vs2, vec_engine)
        .iter_eew()
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |vs2| {
            let result = roundoff_signed(
                vs2 as u128,
                imm5 as u8 & (2 * vec_engine.sew.bit_length() as u8 - 1),
            );

            if sign_mask & result != 0 {
                csr[VXSAT] = 1;
                uint_max as u64
            } else {
                result
            }
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
