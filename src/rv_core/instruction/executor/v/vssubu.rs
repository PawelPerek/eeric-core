use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters, vec_engine: &VectorEngine) {
    let vreg = izip!(v.get(vs2, vec_engine).iter_eew(), v.get(vs1, vec_engine).iter_eew())
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |(vs2, vs1)| {
            match vec_engine.sew.bit_length() {
                8 => (vs2 as  u8).saturating_sub(vs1 as  u8) as u64,
                16 => (vs2 as u16).saturating_sub(vs1 as u16) as u64,
                32 => (vs2 as u32).saturating_sub(vs1 as u32) as u64,
                64 => vs2.saturating_sub(vs1),
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
                8 => (vs2 as u8).saturating_sub(x[rs1] as u8) as u64,
                16 => (vs2 as u16).saturating_sub(x[rs1] as u16) as u64,
                32 => (vs2 as u32).saturating_sub(x[rs1] as u32) as u64,
                64 => vs2.saturating_sub(x[rs1]),
                _ => unimplemented!()
            }
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
