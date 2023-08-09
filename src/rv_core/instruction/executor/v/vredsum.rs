use crate::rv_core::{
    instruction::format::vector::{
        Opmvv
    }, 
    registers::{
        vector::VectorRegisters, 
        integer::IntegerRegisters
    }
};

pub fn vs(Opmvv { dest, vs1, vs2, vm }: Opmvv, v: &mut VectorRegisters) {
    v.apply(dest, v.acquire2(vs2, vs1).map(|(vel2, vel1)| vel2 - vel1));
}