use crate::prelude::*;

use crate::rv_core::{
    instruction::format::Opivi, 
    registers::VectorRegisters
};

pub fn v(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters) {
    todo!()
}