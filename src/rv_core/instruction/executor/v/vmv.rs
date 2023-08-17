use itertools::izip;

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
    let vreg = v.get(vs1).iter_eew().collect();
    
    v.apply(vd, vreg);
}

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    let vreg = v.get(vd).iter_u64().map(|_| x[rs1]).flat_map(u64::to_le_bytes).collect();
    
    v.apply(vd, vreg);
}

pub fn vi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters) {
    let vreg = v.get(vd).iter_u64().map(|_| imm5).flat_map(u64::to_le_bytes).collect();
    
    v.apply(vd, vreg);
}


pub fn xs(Vwxunary0 { dest, vs2, vm, .. }: Vwxunary0, v: &VectorRegisters, x: &mut IntegerRegisters) {
    let first_value = v.get(vs2).iter_eew().next().unwrap();

    x[dest] = first_value;
}

pub fn sx(Vrxunary0 { dest, vs1, vm, .. }: Vrxunary0, v: &mut VectorRegisters, x: &IntegerRegisters) {
    let first_value = u64::to_le_bytes(x[dest]);

    let mut vreg = v.get(vs1);
    let vreg_iter = vreg.iter_eew();
    // Discard first value
    vreg_iter.next().unwrap();

    let new_vreg = first_value[..v.vec_engine.sew().byte_length()].iter().cloned().chain(vreg.collect_bytes()).collect();
    
    v.apply(vs1, new_vreg);
}
