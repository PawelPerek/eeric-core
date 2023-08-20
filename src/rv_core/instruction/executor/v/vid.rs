use crate::prelude::*;

use crate::rv_core::{
    instruction::format::Vmunary0, 
    registers::VectorRegisters
};


pub fn v(Vmunary0 { dest: vd, vs2: _, vm, .. }: Vmunary0, v: &mut VectorRegisters) {
    let vreg = 
        v.get(vd).iter_eew()
        .enumerate()
        .masked_map(
            v.default_mask(vm),
            v.get(vd).iter_eew(),   
            |(i, _)| i as u64
        ).collect_with_eew(v.vec_engine.sew.clone());
    
    v.apply(vd, vreg);
}
