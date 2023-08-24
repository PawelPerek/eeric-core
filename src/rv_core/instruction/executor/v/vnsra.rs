use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{Opivi, Opivv, Opivx},
    registers::{IntegerRegisters, VectorRegisters},
};

use super::narrow_shamt;

pub fn wv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters) {
    let vreg = izip!(v.get_wide(vs2).iter_eew(), v.get(vs1).iter_eew())
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |(vs2, vs1)| {
            (vs2 as i128 >> narrow_shamt(vs1, v.vec_engine.sew.byte_length())) as u64
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn wx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    todo!()
}

pub fn wi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters) {
    todo!()
}
