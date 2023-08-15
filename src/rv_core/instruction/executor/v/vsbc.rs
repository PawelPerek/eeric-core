use crate::rv_core::{
    instruction::format::{
        Opivv,
        Opivx
    }, 
    registers::{
        vector::VectorRegisters, 
        integer::IntegerRegisters
    }
};

pub fn vvm(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters) {
    todo!()
}

pub fn vxm(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    todo!()
}