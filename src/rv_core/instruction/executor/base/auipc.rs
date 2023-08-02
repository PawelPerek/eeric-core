use crate::rv_core::{
    instruction::format::base::U, 
    registers::integer::IntegerRegisters
};

pub fn auipc(U { rd, imm20 }: U, x: &mut IntegerRegisters, pc: u64) {
    x[rd] = pc + imm20 << 12;
}