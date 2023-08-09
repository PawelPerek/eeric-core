use crate::rv_core::{
    instruction::format::base::I, 
    registers::integer::IntegerRegisters
};

pub fn slti(I { rd, rs, imm12 }: I, x: &mut IntegerRegisters) {
    x[rd] = if x[rs] < imm12 { 1 } else { 0 };
}