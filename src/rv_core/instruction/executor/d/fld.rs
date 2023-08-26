use crate::rv_core::{
    instruction::format::I,
    memory::Memory,
    registers::{FloatRegisters, IntegerRegisters},
};

pub fn fld(I { rd, rs1, imm12 }: I, x: &IntegerRegisters, f: &mut FloatRegisters, mem: &Memory) {
    let addr = x[rs1] + imm12;
    let fp = f64::from_le_bytes(mem.get(addr as usize));

    f[rd] = fp;
}
