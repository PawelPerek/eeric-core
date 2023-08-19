use crate::prelude::*;

use crate::rv_core::{
    instruction::format::{Opivi, Opivv}, 
    registers::VectorRegisters
};

pub fn v(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters) {
    for i in 0..4 {
        // Note: vs1 = vs2
        super::vmv::vv(Opivv { vd: vd + i, vs1: vs2 + i, vs2: 0, vm }, v)
    }
}