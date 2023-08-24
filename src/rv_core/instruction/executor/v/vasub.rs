use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{Opmvv, Opmvx},
    registers::{IntegerRegisters, VectorRegisters},
};

pub fn vv(Opmvv { dest, vs1, vs2, vm }: Opmvv, v: &mut VectorRegisters) {
    todo!()
}

pub fn vx(Opmvx { dest, rs1, vs2, vm }: Opmvx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    todo!()
}
