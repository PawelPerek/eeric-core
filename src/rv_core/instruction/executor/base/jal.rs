use crate::rv_core::{instruction::format::U, registers::IntegerRegisters};

pub fn jal(U { rd, imm20 }: U, x: &mut IntegerRegisters, pc: &mut u64) {
    x[rd] = *pc + 4;
    *pc = pc.wrapping_add(imm20);
}
