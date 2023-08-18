use crate::prelude::*;

use crate::rv_core::{
    instruction::format::Vmunary0, 
    registers::VectorRegisters
};


pub fn m(Vmunary0 { dest: vd, vs2, vm, .. }: Vmunary0, v: &mut VectorRegisters) {
    let mut mask = v.get(vs2);

    let mut prefix_sum = 0u64;
    
    let vreg = mask
        .iter_eew()
        .map(|val| {
            if val != 0 {
                prefix_sum += 1;
            }
            prefix_sum
        })
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
