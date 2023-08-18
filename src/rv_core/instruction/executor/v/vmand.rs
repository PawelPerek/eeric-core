use crate::prelude::*;

use crate::rv_core::{
    instruction::format::Opmvv, 
    registers::VectorRegisters
};

pub fn mm(Opmvv { dest, vs1, vs2, vm }: Opmvv, v: &mut VectorRegisters) {
    todo!()
}
