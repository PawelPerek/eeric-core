use num_traits::{Float, ToPrimitive};



use crate::rv_core::instruction::executor::prelude::*;

pub fn xufw(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorRegisters,
) {
    let vreg = v
        .get_wide(vs2)
        .iter_fp()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            vs2.round().to_u64().unwrap()
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn xfw(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorRegisters,
) {
    let vreg = v
        .get_wide(vs2)
        .iter_fp()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            vs2.round().to_i64().unwrap() as u64
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn fxuw(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorRegisters,
) {
    let vreg = v
        .get_wide(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_fp(), |vs2| {
            match v.vec_engine.sew.bit_length() {
                32 => ArbitraryFloat::F32(vs2 as u32 as f32),
                64 => ArbitraryFloat::F64(vs2 as u64 as f64),
                _ => unimplemented!("Floating point is supported only fow SEW=32 or SEW=64"),
            }
        })
        .collect_fp();

    v.apply(vd, vreg);
}

pub fn fxw(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorRegisters,
) {
    let vreg = v
        .get_wide(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_fp(), |vs2| {
            match v.vec_engine.sew.bit_length() {
                32 => ArbitraryFloat::F32(vs2 as i32 as f32),
                64 => ArbitraryFloat::F64(vs2 as i64 as f64),
                _ => unimplemented!("Floating point is supported only fow SEW=32 or SEW=64"),
            }
        })
        .collect_fp();

    v.apply(vd, vreg);
}

pub fn ffw(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorRegisters,
) {
    let vreg = v
        .get_wide(vs2)
        .iter_fp()
        .masked_map(v.default_mask(vm), v.get(vd).iter_fp(), |vs2| {
            vs2.half_precision()
        })
        .collect_fp();

    v.apply(vd, vreg);
}

pub fn rodffw(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorRegisters,
) {
    todo!()
}

pub fn rtzxufw(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorRegisters,
) {
    let vreg = v
        .get_wide(vs2)
        .iter_fp()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            vs2.to_u64().unwrap()
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn rtzxfw(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorRegisters,
) {
    let vreg = v
        .get_wide(vs2)
        .iter_fp()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            vs2.to_i64().unwrap() as u64
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
