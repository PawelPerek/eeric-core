use crate::rv_core::{instruction::format::I, memory::Memory, registers::IntegerRegisters};

pub fn lbu(I { rd, rs1, imm12 }: I, x: &mut IntegerRegisters, mem: &Memory) {
    let addr = x[rs1] + imm12;
    let int = u8::from_le_bytes(mem.get(addr as usize));

    x[rd] = int as u64;
}
