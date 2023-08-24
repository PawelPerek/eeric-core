use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{Opivi, Opivv, Opivx},
    registers::{CsrRegisters, IntegerRegisters, VectorRegisters},
};

use super::utils::rounding::Roundoff;

pub fn wv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters, csr: &mut CsrRegisters) {
    let roundoff_signed = Roundoff::new_signed(csr);

    let uint_max = u64::MAX >> (64 - v.vec_engine.sew.bit_length());
    let sign_mask = u64::MIN << v.vec_engine.sew.bit_length();

    let vreg = izip!(v.get(vs2).iter_eew(), v.get(vs1).iter_eew())
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |(vs2, vs1)| {
            let result = roundoff_signed(vs2 as u128, vs1 as u8 & (2 * v.vec_engine.sew.bit_length() as u8 - 1));

            if sign_mask & result != 0 {
                csr[VXSAT] = 1;
                uint_max as u64
            } else {
                result
            }
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn wx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters, csr: &mut CsrRegisters) {
    let roundoff_signed = Roundoff::new_signed(csr);

    let uint_max = u64::MAX >> (64 - v.vec_engine.sew.bit_length());
    let sign_mask = u64::MIN << v.vec_engine.sew.bit_length();

    let vreg = v.get(vs2).iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            let result = roundoff_signed(vs2 as u128, x[rs1] as u8 & (2 * v.vec_engine.sew.bit_length() as u8 - 1));

            if sign_mask & result != 0 {
                csr[VXSAT] = 1;
                uint_max as u64
            } else {
                result
            }
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn wi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters, csr: &mut CsrRegisters) {
    let roundoff_signed = Roundoff::new_signed(csr);

    let uint_max = u64::MAX >> (64 - v.vec_engine.sew.bit_length());
    let sign_mask = u64::MIN << v.vec_engine.sew.bit_length();

    let vreg = v.get(vs2).iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            let result = roundoff_signed(vs2 as u128, imm5 as u8 & (2 * v.vec_engine.sew.bit_length() as u8 - 1));

            if sign_mask & result != 0 {
                csr[VXSAT] = 1;
                uint_max as u64
            } else {
                result
            }
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
