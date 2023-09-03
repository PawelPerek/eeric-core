use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    vsx: Vsx,
    eew: SEW,
    v: &VectorRegisters,
    vec_engine: &VectorEngine,
    x: &IntegerRegisters,
    mem: &mut Memory,
) {
    super::vsox::v(vsx, eew, v, vec_engine, x, mem)
}
