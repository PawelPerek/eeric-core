use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{Opfvf, Opfvv},
    registers::{FloatRegisters, VectorRegisters},
};

pub fn vv(Opfvv { dest: vd, vs1, vs2, vm }: Opfvv, v: &mut VectorRegisters) {
    let vreg = izip!(v.get(vs2).iter_fp(), v.get(vs1).iter_fp(), v.get_wide(vd).iter_fp())
        .masked_map(
            v.default_mask(vm),
            v.get_wide(vd).iter_fp(),
            |(vs2, vs1, vd)| vs2.double_precision() * vs1.double_precision() + vd,
        )
        .collect_fp();

    v.apply(vd, vreg);
}

pub fn vf(Opfvf { vd, rs1, vs2, vm }: Opfvf, v: &mut VectorRegisters, f: &FloatRegisters) {
    let vreg = izip!(v.get(vs2).iter_fp(), v.get(vd).iter_fp())
        .masked_map(
            v.default_mask(vm),
            v.get_wide(vd).iter_fp(),
            |(vs2, vd)| {
                let vs2 = vs2.double_precision();
                vs2 * ArbitraryFloat::copy_type(&vs2, f[rs1]) + vd
            }
        )
        .collect_fp();

    v.apply(vd, vreg);
}
