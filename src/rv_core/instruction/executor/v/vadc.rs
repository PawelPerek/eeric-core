use itertools::izip;

use crate::extensions::iter_mask_ext::IterMaskExt;

use crate::rv_core::{
    instruction::format::{
        Opivv,
        Opivx,
        Opivi,
    }, 
    registers::{
        VectorRegisters, 
        IntegerRegisters
    }
};

pub fn vvm(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters) {
    let vreg = izip!(
        v.get(vs1).iter_eew(),
        v.get(vs2).iter_eew()
    ).masked_map(
        v.get(0).iter_mask(),
        v.get(vd).iter_eew(),
        |vel:(u64, u64), m| vel.0 + vel.1 + m
    ).flat_map(u64::to_le_bytes).collect();

    v.apply(vd, vreg);
}

pub fn vxm(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    let vreg = izip!(
        v.get(vs2).iter_eew()
    ).masked_map(
        v.get(0).iter_mask(),
        v.get(vd).iter_eew(),
        |vel, m| vel + x[rs1] + m
    ).flat_map(u64::to_le_bytes).collect();

    v.apply(vd, vreg);
}

pub fn vim(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters) {
    let vreg = izip!(
        v.get(vs2).iter_eew()
    ).masked_map(
        v.get(0).iter_mask(),
        v.get(vd).iter_eew(),
        |vel, m| vel + imm5 + m
    ).flat_map(u64::to_le_bytes).collect();

    v.apply(vd, vreg);
}
