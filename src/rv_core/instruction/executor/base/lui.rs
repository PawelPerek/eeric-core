use crate::rv_core::{
    instruction::format::base::U, 
    registers::integer::IntegerRegisters
};

pub fn lui(U { rd, imm20 }: U, x: &mut IntegerRegisters) {
    x[rd] = imm20 << 12;
}