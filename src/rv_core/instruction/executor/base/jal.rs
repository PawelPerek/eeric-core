use crate::rv_core::instruction::executor::prelude::*;

pub fn jal(U { rd, imm20 }: U, x: &mut IntegerRegisters, pc: &mut u64) {
    x[rd] = pc.wrapping_add(4);
    *pc = pc.wrapping_add(imm20 as u64);
}
