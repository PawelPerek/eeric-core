use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    vsx: Vsx,
    eew: SEW,
    nf: usize,
    v: &VectorRegisters,
    vec_engine: &VectorEngine,
    x: &IntegerRegisters,
    mem: &mut Memory,
) {
    super::vsoxseg::v(vsx, eew, nf, v, vec_engine, x, mem)
}
