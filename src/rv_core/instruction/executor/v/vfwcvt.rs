use num_traits::{Float, ToPrimitive};

use crate::rv_core::instruction::executor::prelude::*;

pub fn xufv(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
) {
    let vreg = v
        .get(vs2, vec_engine)
        .iter_fp()
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get_wide(vd, vec_engine).iter_eew(),
            |vs2| vs2.round().to_u128().unwrap(),
        )
        .collect_with_wide_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn xfv(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
) {
    let vreg = v
        .get(vs2, vec_engine)
        .iter_fp()
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get_wide(vd, vec_engine).iter_eew(),
            |vs2| vs2.round().to_i128().unwrap() as u128,
        )
        .collect_with_wide_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn fxuv(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
) {
    let vreg = v
        .get(vs2, vec_engine)
        .iter_eew()
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get_wide(vd, vec_engine).iter_fp(),
            |vs2| match vec_engine.sew.bit_length() {
                16 => ArbitraryFloat::F32(vs2 as u16 as f32),
                32 => ArbitraryFloat::F64(vs2 as u32 as f64),
                _ => unimplemented!("Floating point is supported only fow SEW=32 or SEW=64"),
            },
        )
        .collect_fp();

    v.apply(vd, vreg, vec_engine);
}

pub fn fxv(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
) {
    let vreg = v
        .get(vs2, vec_engine)
        .iter_eew()
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get_wide(vd, vec_engine).iter_fp(),
            |vs2| match vec_engine.sew.bit_length() {
                16 => ArbitraryFloat::F32(vs2 as i16 as f32),
                32 => ArbitraryFloat::F64(vs2 as i32 as f64),
                _ => unimplemented!("Floating point is supported only fow SEW=32 or SEW=64"),
            },
        )
        .collect_fp();

    v.apply(vd, vreg, vec_engine);
}

pub fn ffv(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
) {
    let vreg = v
        .get_wide(vs2, vec_engine)
        .iter_fp()
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get(vd, vec_engine).iter_fp(),
            |vs2| vs2.double_precision(),
        )
        .collect_fp();

    v.apply(vd, vreg, vec_engine);
}

pub fn rtzxufv(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
) {
    let vreg = v
        .get(vs2, vec_engine)
        .iter_fp()
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get_wide(vd, vec_engine).iter_eew(),
            |vs2| vs2.to_u128().unwrap(),
        )
        .collect_with_wide_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn rtzxfv(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
) {
    let vreg = v
        .get(vs2, vec_engine)
        .iter_fp()
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get_wide(vd, vec_engine).iter_eew(),
            |vs2| vs2.to_i128().unwrap() as u128,
        )
        .collect_with_wide_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
