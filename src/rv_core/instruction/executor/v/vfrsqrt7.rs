use num_traits::Float;

use crate::prelude::*;

use crate::rv_core::{instruction::format::Vfunary1, registers::VectorRegisters};

pub fn v(
    Vfunary1 {
        dest: vd, vs2, vm, ..
    }: Vfunary1,
    v: &mut VectorRegisters,
) {
    let vreg = v
        .get(vs2)
        .iter_fp()
        .masked_map(v.default_mask(vm), v.get(vd).iter_fp(), |vs2| {
            ArbitraryFloat::copy_type(&vs2, 1.0) / vs2.sqrt()
        })
        .collect_fp();

    v.apply(vd, vreg);
}
