use crate::rv_core::{
    instruction::format::base::I, 
    registers::integer::IntegerRegisters, 
    memory::Memory
};

pub fn lh(I { rd, rs, imm12 }: I, x: &mut IntegerRegisters, memory: &Memory) {
    let address: usize = (x[rs] + imm12) as usize;
    let value = u16::from_le_bytes(memory.0[address .. address + 2].try_into().unwrap());
    x[rd] = value as u64;
}