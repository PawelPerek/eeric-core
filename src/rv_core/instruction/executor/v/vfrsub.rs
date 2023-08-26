use crate::prelude::*;

use crate::rv_core::{
    instruction::format::Opfvf,
    registers::{FloatRegisters, VectorRegisters},
};

pub fn vf(Opfvf { vd, rs1, vs2, vm }: Opfvf, v: &mut VectorRegisters, f: &FloatRegisters) {
    let vreg = v.get(vs2).iter_fp()
        .masked_map(v.default_mask(vm), v.get(vd).iter_fp(), |vs2| {
            ArbitraryFloat::copy_type(&vs2, f[rs1]) - vs2
        })
        .collect_fp();

    v.apply(vd, vreg);
}
