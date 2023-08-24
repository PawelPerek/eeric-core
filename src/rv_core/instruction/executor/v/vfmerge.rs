use crate::prelude::*;

use crate::rv_core::{
    instruction::format::Opfvf,
    registers::{FloatRegisters, VectorRegisters},
};

pub fn vfm(Opfvf { vd, rs1, vs2, vm }: Opfvf, v: &mut VectorRegisters, f: &FloatRegisters) {
    todo!()
}
