use crate::rv_core::instruction::executor::prelude::*;

pub fn vf2(
    Vxunary0 {
        dest: vd, vs2, vm, ..
    }: Vxunary0,
    v: &mut VectorRegisters, vec_engine: &VectorEngine,
) {
    let vreg = v
        .get(vs2, vec_engine)
        .iter_eew_div_2()
        .take(vec_engine.vlmax() / 2)
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |vs2| {
            match vec_engine.sew.bit_length() {
                16 => vs2 as i8 as u64,
                32 => vs2 as i16 as u64,
                64 => vs2 as i32 as u64,
                128 => vs2,
                // sew.half().unwrap() will panic if sew = 8b
                _ => unreachable!(),
            }
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn vf4(
    Vxunary0 {
        dest: vd, vs2, vm, ..
    }: Vxunary0,
    v: &mut VectorRegisters, vec_engine: &VectorEngine,
) {
    let vreg = v
        .get(vs2, vec_engine)
        .iter_eew_div_4()
        .take(vec_engine.vlmax() / 4)
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |vs2| {
            match vec_engine.sew.bit_length() {
                32 => vs2 as i8 as u64,
                64 => vs2 as i16 as u64,
                128 => vs2 as i32 as u64,
                // sew.fourth().unwrap() will panic if sew = 8b or 16b
                _ => unreachable!(),
            }
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn vf8(
    Vxunary0 {
        dest: vd, vs2, vm, ..
    }: Vxunary0,
    v: &mut VectorRegisters, vec_engine: &VectorEngine,
) {
    let vreg = v
        .get(vs2, vec_engine)
        .iter_eew_div_8()
        .take(vec_engine.vlmax() / 8)
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |vs2| {
            match vec_engine.sew.bit_length() {
                64 => vs2 as i8 as u64,
                128 => vs2 as i16 as u64,
                // sew.eight().unwrap() will panic if sew = 8b, 16b or 32b
                _ => unreachable!(),
            }
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
