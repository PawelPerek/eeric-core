use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters) {
    let vreg = izip!(v.get(vs2).iter_eew(), v.get(vs1).iter_eew())
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |(vs2, vs1)| {
            match v.vec_engine.sew.bit_length() {
                8 => (vs2 as  u8).saturating_add(vs1 as  u8) as u64,
                16 => (vs2 as u16).saturating_add(vs1 as u16) as u64,
                32 => (vs2 as u32).saturating_add(vs1 as u32) as u64,
                64 => vs2.saturating_add(vs1),
                _ => unimplemented!()
            }
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            match v.vec_engine.sew.bit_length() {
                8 => (vs2 as u8).saturating_add(x[rs1] as u8) as u64,
                16 => (vs2 as u16).saturating_add(x[rs1] as u16) as u64,
                32 => (vs2 as u32).saturating_add(x[rs1] as u32) as u64,
                64 => vs2.saturating_add(x[rs1]),
                _ => unimplemented!()
            }
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn vi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters) {
    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            match v.vec_engine.sew.bit_length() {
                8 => (vs2 as u8).saturating_add(imm5 as u8) as u64,
                16 => (vs2 as u16).saturating_add(imm5 as u16) as u64,
                32 => (vs2 as u32).saturating_add(imm5 as u32) as u64,
                64 => vs2.saturating_add(imm5 as u64),
                _ => unimplemented!()
            }
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
