use crate::prelude::*;

use crate::rv_core::{
    instruction::format::Opmvx, 
    registers::{VectorRegisters, IntegerRegisters}
};

pub fn vx(Opmvx { dest, rs1, vs2, vm }: Opmvx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    todo!()
}
