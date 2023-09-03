use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    vlx: Vlx,
    eew: SEW,
    nf: usize,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
    x: &IntegerRegisters,
    mem: &Memory,
) {
    super::vloxseg::v(
        vlx,
        eew,
        nf,
        v,
        vec_engine,
        x,
        mem
    )
}
