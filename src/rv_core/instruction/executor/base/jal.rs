use crate::rv_core::{
    instruction::format::base::U, 
    registers::integer::IntegerRegisters
};

pub fn jal(U { rd, imm20 }: U, x: &IntegerRegisters) {
    todo!()
}