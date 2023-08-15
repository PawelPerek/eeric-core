use crate::rv_core::{
    instruction::format::Opfvv, 
    registers::VectorRegisters
};

pub fn vs(Opfvv { dest, vs1, vs2, vm }: Opfvv, v: &mut VectorRegisters) {
    todo!()
}