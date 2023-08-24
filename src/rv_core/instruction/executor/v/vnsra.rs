use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{Opivi, Opivv, Opivx},
    registers::{IntegerRegisters, VectorRegisters},
};

pub fn wv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters) {
    todo!()
}

pub fn wx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    todo!()
}

pub fn wi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters) {
    todo!()
}
