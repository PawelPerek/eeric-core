use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{Opivi, Opivv, Opivx, Vrxunary0, Vwxunary0},
    registers::{IntegerRegisters, VectorRegisters},
};

pub fn vv(
    Opivv {
        vd,
        vs1,
        vs2: _,
        vm: _,
    }: Opivv,
    v: &mut VectorRegisters,
) {
    let vreg = v
        .get(vs1)
        .iter_eew()
        .map(|vs1| vs1)
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn vx(
    Opivx {
        vd,
        rs1,
        vs2: _,
        vm: _,
    }: Opivx,
    v: &mut VectorRegisters,
    x: &IntegerRegisters,
) {
    let vreg = v
        .get(vd)
        .iter_eew()
        .map(|_| x[rs1])
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn vi(
    Opivi {
        vd,
        imm5,
        vs2: _,
        vm: _,
    }: Opivi,
    v: &mut VectorRegisters,
) {
    let vreg = v
        .get(vd)
        .iter_eew()
        .map(|_| imm5)
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn xs(
    Vwxunary0 {
        dest: rd, vs2, vm: _, ..
    }: Vwxunary0,
    v: &VectorRegisters,
    x: &mut IntegerRegisters,
) {
    let first_value = v.get(vs2).iter_eew().next().unwrap();

    x[rd] = first_value;
}

pub fn sx(
    Vrxunary0 {
        dest: vd, rs1, vm: _, ..
    }: Vrxunary0,
    v: &mut VectorRegisters,
    x: &IntegerRegisters,
) {
    let first_value = u64::to_le_bytes(x[rs1]);

    let vreg = v.get(vd);
    let mut vreg_data = vreg.iter_byte().collect_vec();

    for i in 0..v.vec_engine.sew.byte_length() {
        vreg_data[i] = first_value[i];
    }

    v.apply(vd, vreg_data.into_iter().collect());
}
