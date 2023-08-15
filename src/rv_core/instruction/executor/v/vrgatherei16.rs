use crate::rv_core::{
    instruction::format::Opivv, 
    registers::vector::VectorRegisters
};

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters) {
    let vlmax = v.vec_engine.vlmax();

    v.apply(vd, v.acquire_i16(vs1).map(|index| {
        if index as usize >= vlmax { 
            0 
        } else { 
            v.get(vs2).iter_eew().nth(index as usize).unwrap()
        }
    }));
}
