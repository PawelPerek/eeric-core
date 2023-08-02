use crate::rv_core::{
    instruction::format::base::I, 
    registers::integer::IntegerRegisters
};

pub fn sltiu(I { rd, rs, imm12 }: I, x: &mut IntegerRegisters) {
    let rs = x[rs] as i64;
    x[rd] = if rs < imm12 as i64 { 1 } else { 0 };
}