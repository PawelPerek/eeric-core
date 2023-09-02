use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vsx { vs3, rs1, vs2, vm }: Vsx,
    eew: usize,
    nf: usize,
    v: &VectorRegisters,
    vec_engine: &VectorEngine,
    mem: &mut Memory,
) {
    todo!()
}
