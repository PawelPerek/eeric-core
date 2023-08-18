use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{
        Opfvv,
        Opfvf
    }, 
    registers::{
        VectorRegisters, 
        FloatRegisters
    }
};

pub fn vv(Opfvv { dest, vs1, vs2, vm }: Opfvv, v: &mut VectorRegisters) {
    todo!()
}

pub fn vf(Opfvf { vd, rs1, vs2, vm }: Opfvf, v: &mut VectorRegisters, f: &FloatRegisters) {
    todo!()
}
