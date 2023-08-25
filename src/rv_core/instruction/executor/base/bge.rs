use crate::rv_core::{instruction::format::S, registers::IntegerRegisters};

pub fn bge(S { rs1, rs2, imm12 }: S, x: &IntegerRegisters, pc: &mut u64) {
    if x[rs1] as i64 >= x[rs2] as i64 {
        *pc = pc.wrapping_add(imm12);
    }
}

