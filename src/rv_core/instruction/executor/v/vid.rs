use crate::rv_core::{
    instruction::format::Vmunary0, 
    registers::VectorRegisters
};


pub fn v(Vmunary0 { dest: vd, vs2, vm, .. }: Vmunary0, v: &mut VectorRegisters) {
    let vreg = v
        .get(vd)
        .iter_eew()
        .enumerate()
        .map(|(i, _)| i as u64)
        .flat_map(u64::to_le_bytes)
        .collect();
    
    v.apply(vd, vreg);
}
