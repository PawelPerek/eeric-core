use crate::prelude::*;

use crate::rv_core::{
    instruction::format::Vwxunary0, 
    registers::{VectorRegisters, IntegerRegisters}
};


pub fn m(Vwxunary0 { dest: rd, vs2, vm, .. }: Vwxunary0, v: &VectorRegisters, x: &mut IntegerRegisters) {
    let mask_count = izip!(
        v.get(0).iter_mask(),
        v.get(vs2).iter_mask()
    )
        .filter(|&(vs2_mask, v0_mask)| vs2_mask == 1 && v0_mask == 1)
        .count();
    
    x[rd] = mask_count as u64;
}
