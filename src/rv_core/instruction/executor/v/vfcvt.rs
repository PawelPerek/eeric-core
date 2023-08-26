use num_traits::{ToPrimitive, Float};

use crate::prelude::*;

use crate::rv_core::{instruction::format::Vfunary0, registers::VectorRegisters};

pub fn xufv(Vfunary0 { dest: vd, vs2, vm, .. }: Vfunary0, v: &mut VectorRegisters) {
    let vreg = v.get(vs2).iter_fp()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            vs2.round().to_u64().unwrap()
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn xfv(Vfunary0 { dest: vd, vs2, vm, .. }: Vfunary0, v: &mut VectorRegisters) {
    let vreg = v.get(vs2).iter_fp()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            vs2.round().to_i64().unwrap() as u64
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn fxuv(Vfunary0 { dest: vd, vs2, vm, .. }: Vfunary0, v: &mut VectorRegisters) {
    let vreg = v.get(vs2).iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_fp(), |vs2| {
            match v.vec_engine.sew.bit_length() {
                32 => ArbitraryFloat::F32(vs2 as f32),
                64 => ArbitraryFloat::F64(vs2 as f64),
                _ => unreachable!(),
            }
        })
        .collect_fp();

    v.apply(vd, vreg);
}

pub fn fxv(Vfunary0 { dest: vd, vs2, vm, .. }: Vfunary0, v: &mut VectorRegisters) {
    let vreg = v.get(vs2).iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_fp(), |vs2| {
            match v.vec_engine.sew.bit_length() {
                32 => ArbitraryFloat::F32(vs2 as i64 as f32),
                64 => ArbitraryFloat::F64(vs2 as i64 as f64),
                _ => unreachable!(),
            }
        })
        .collect_fp();

    v.apply(vd, vreg);
}

pub fn rtzxufv(Vfunary0 { dest: vd, vs2, vm, .. }: Vfunary0, v: &mut VectorRegisters) {
    let vreg = v.get(vs2).iter_fp()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            vs2.to_u64().unwrap()
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn rtzxfv(Vfunary0 { dest: vd, vs2, vm, .. }: Vfunary0, v: &mut VectorRegisters) {
    let vreg = v.get(vs2).iter_fp()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            vs2.to_i64().unwrap() as u64
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
