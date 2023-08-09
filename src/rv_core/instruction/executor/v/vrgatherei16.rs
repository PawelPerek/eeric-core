use crate::rv_core::{
    instruction::format::vector::{
        Opivv,
        Opivx,
        Opivi,
    }, 
    registers::{
        vector::VectorRegisters, 
        integer::IntegerRegisters
    }
};

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters) {
    let vlmax = v.vec_engine.vlmax();

    v.apply(vd, v.acquirei16(vs1).map(|index| {
        if index as usize >= vlmax { 
            0 
        } else { 
            v.get(vs2).nth(index as usize).unwrap()
        }
    }));
}
