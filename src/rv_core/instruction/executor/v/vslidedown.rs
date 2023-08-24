use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{Opivi, Opivx},
    registers::{IntegerRegisters, VectorRegisters},
};

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    todo!()
}

pub fn vi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters) {
    todo!()
}
