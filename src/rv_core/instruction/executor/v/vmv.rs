use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{
        Opivv,
        Opivx,
        Opivi,
        Vwxunary0,
        Vrxunary0
    }, 
    registers::{
        VectorRegisters, 
        IntegerRegisters
    }
};

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters) {
    let vreg = 
        v.get(vs2).iter_eew()
        .masked_map(
            v.get(0).iter_mask(),
            v.get(vd).iter_eew(),
            |vel| vel
        ).collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    let vreg = 
        v.get(vd).iter_eew()
        .masked_map(
            v.get(0).iter_mask(),
            v.get(vd).iter_eew(),
            |_| x[rs1]
        ).collect_with_eew(v.vec_engine.sew.clone());
    
    v.apply(vd, vreg);
}

pub fn vi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters) {
    let vreg = 
        v.get(vd).iter_eew()
        .masked_map(
            v.get(0).iter_mask(),
            v.get(vd).iter_eew(),
            |_| imm5
        ).collect_with_eew(v.vec_engine.sew.clone());
    
    v.apply(vd, vreg);
}


pub fn xs(Vwxunary0 { dest, vs2, vm, .. }: Vwxunary0, v: &VectorRegisters, x: &mut IntegerRegisters) {
    let first_value = v.get(vs2).iter_eew().next().unwrap();

    x[dest] = first_value;
}

pub fn sx(Vrxunary0 { dest, vs1, vm, .. }: Vrxunary0, v: &mut VectorRegisters, x: &IntegerRegisters) {
    let first_value = u64::to_le_bytes(x[dest]);

    let vreg = v.get(vs1);
    let mut vreg_iter = vreg.iter_eew();
    // Discard the first value
    vreg_iter.next().unwrap();

    let new_vreg = first_value[..v.vec_engine.sew().byte_length()]
        .into_iter()
        .copied()  
        .chain(vreg_iter.collect_with_eew(v.vec_engine.sew.clone()).iter_byte())
        .collect();
    
    v.apply(vs1, new_vreg);
}
