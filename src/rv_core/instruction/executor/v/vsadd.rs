use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters, vec_engine: &VectorEngine) {
    let vreg = izip!(v.get(vs2, vec_engine).iter_eew(), v.get(vs1, vec_engine).iter_eew())
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |(vs2, vs1)| {
            match vec_engine.sew.bit_length() {
                8 => (vs2 as i8).saturating_add(vs1 as i8) as u64,
                16 => (vs2 as i16).saturating_add(vs1 as i16) as u64,
                32 => (vs2 as i32).saturating_add(vs1 as i32) as u64,
                64 => (vs2 as i64).saturating_add(vs1 as i64) as u64,
                _ => unimplemented!()
            }
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, vec_engine: &VectorEngine, x: &IntegerRegisters) {
    let vreg = v
        .get(vs2, vec_engine)
        .iter_eew()
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |vs2| {
            match vec_engine.sew.bit_length() {
                8 => (vs2 as i8).saturating_add(x[rs1] as i8) as u64,
                16 => (vs2 as i16).saturating_add(x[rs1] as i16) as u64,
                32 => (vs2 as i32).saturating_add(x[rs1] as i32) as u64,
                64 => (vs2 as i64).saturating_add(x[rs1] as i64) as u64,
                _ => unimplemented!()
            }
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn vi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters, vec_engine: &VectorEngine) {
    let vreg = v
        .get(vs2, vec_engine)
        .iter_eew()
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |vs2| {
            match vec_engine.sew.bit_length() {
                8 => (vs2 as i8).saturating_add(imm5 as i8) as u64,
                16 => (vs2 as i16).saturating_add(imm5 as i16) as u64,
                32 => (vs2 as i32).saturating_add(imm5 as i32) as u64,
                64 => (vs2 as i64).saturating_add(imm5 as i64) as u64,
                _ => unimplemented!()
            }
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
