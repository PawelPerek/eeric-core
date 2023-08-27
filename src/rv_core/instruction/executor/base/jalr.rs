use crate::rv_core::instruction::executor::prelude::*;

pub fn jalr(I { rd, rs1, imm12 }: I, x: &mut IntegerRegisters, pc: &mut u64) {
    let t = *pc + 4;
    *pc = (x[rs1] + imm12) & !1;
    x[rd] = t;
}
