use crate::rv_core::{instruction::format::I, registers::IntegerRegisters};

pub fn jalr(I { rd, rs1: rs, imm12 }: I, x: &mut IntegerRegisters, pc: &mut u64) {
    let t = *pc + 4;
    *pc = (x[rs] + imm12) & !1;
    x[rd] = t;
}
