use crate::rv_core::instruction::executor::prelude::*;

pub fn auipc(U { rd, imm20 }: U, x: &mut IntegerRegisters, pc: u64) {
    x[rd] = pc + imm20 << 12;
}
