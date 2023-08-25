use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{Opfvf, Opfvv},
    registers::{FloatRegisters, VectorRegisters},
};

pub fn vv(Opfvv { dest: vd, vs1, vs2, vm }: Opfvv, v: &mut VectorRegisters) {
    let vreg = izip!(v.get(vs2).iter_fp(), v.get(vs1).iter_fp())
        .masked_map(v.default_mask(vm), v.get(vd).iter_fp(), |(vs2, vs1)| {
            vs2 / vs1
        })
        .collect_fp();

    v.apply(vd, vreg);
}

pub fn vf(Opfvf { vd, rs1, vs2, vm }: Opfvf, v: &mut VectorRegisters, f: &FloatRegisters) {
    let vreg = v.get(vs2).iter_fp()
        .masked_map(v.default_mask(vm), v.get(vd).iter_fp(), |vs2| {
            vs2 / ArbitraryFloat::F64(f[rs1])
        })
        .collect_fp();

    v.apply(vd, vreg);
}