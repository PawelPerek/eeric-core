use crate::rv_core::{
    instruction::format::base::I, 
    registers::integer::IntegerRegisters
};

pub fn jalr(I { rd, rs, imm12 }: I, x: &IntegerRegisters) {
    todo!()
}