use crate::rv_core::instruction::executor::prelude::*;

pub fn jal(U { rd, imm20 }: U, x: &mut IntegerRegisters, pc: &mut u64) {
    x[rd] = *pc + 4;
    *pc += imm20 as u64;
}
