use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vsr { vs3, rs1 }: Vsr,
    nf: usize,
    v: &VectorRegisters,
    vec_engine: &VectorEngine,
    mem: &mut Memory,
) {
    todo!()
}
