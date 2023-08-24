use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{Opfvf, Vrfunary0, Vwfunary0},
    registers::{FloatRegisters, VectorRegisters},
};

pub fn vf(Opfvf { vd, rs1, vs2, vm }: Opfvf, v: &mut VectorRegisters, f: &FloatRegisters) {
    todo!()
}

pub fn sf(Vrfunary0 { vd, rs1, vm, .. }: Vrfunary0, v: &mut VectorRegisters, f: &FloatRegisters) {
    todo!()
}

pub fn fs(Vwfunary0 { dest, vs2, vm, .. }: Vwfunary0, v: &mut VectorRegisters, f: &FloatRegisters) {
    todo!()
}
