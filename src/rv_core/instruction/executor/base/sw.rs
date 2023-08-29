use crate::rv_core::instruction::executor::prelude::*;

pub fn sw(S { rs1, rs2, imm12 }: S, x: &IntegerRegisters, mem: &mut Memory) {
    let addr = x[rs1] + imm12 as u64;
    let bytes = (x[rs2] as u32).to_le_bytes();

    mem.set(addr as usize, bytes);
}
