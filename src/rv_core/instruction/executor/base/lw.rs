use crate::rv_core::instruction::executor::prelude::*;

pub fn lw(I { rd, rs1, imm12 }: I, x: &mut IntegerRegisters, mem: &Memory) {
    let addr = x[rs1] + imm12 as u64;
    let int = i32::from_le_bytes(mem.get(addr as usize));

    x[rd] = int as u64;
}
