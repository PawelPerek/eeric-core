use crate::rv_core::{
    instruction::format::Opmvv, 
    registers::vector::VectorRegisters
};

pub fn vs(Opmvv { dest, vs1, vs2, vm }: Opmvv, v: &mut VectorRegisters) {
    todo!()
}
