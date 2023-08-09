use crate::rv_core::{
    instruction::format::base::I, 
    registers::integer::IntegerRegisters
};

pub fn andi(I { rd, rs, imm12 }: I, x: &mut IntegerRegisters) {
    x[rd] = x[rs] & imm12;
}