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
            v.get(vd, vec_engine).iter_eew(),
            |vs2| vs2.round().to_u64().unwrap(),
        )
        .collect_with_eew(vec_engine.sew.clone());

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
            v.get(vd, vec_engine).iter_eew(),
            |vs2| vs2.round().to_i64().unwrap() as u64,
        )
        .collect_with_eew(vec_engine.sew.clone());

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
            v.get(vd, vec_engine).iter_fp(),
            |vs2| match vec_engine.sew.bit_length() {
                32 => ArbitraryFloat::F32(vs2 as f32),
                64 => ArbitraryFloat::F64(vs2 as f64),
                _ => unreachable!(),
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
            v.get(vd, vec_engine).iter_fp(),
            |vs2| match vec_engine.sew.bit_length() {
                32 => ArbitraryFloat::F32(vs2 as i64 as f32),
                64 => ArbitraryFloat::F64(vs2 as i64 as f64),
                _ => unreachable!(),
            },
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
            v.get(vd, vec_engine).iter_eew(),
            |vs2| vs2.to_u64().unwrap(),
        )
        .collect_with_eew(vec_engine.sew.clone());

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
            v.get(vd, vec_engine).iter_eew(),
            |vs2| vs2.to_i64().unwrap() as u64,
        )
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
