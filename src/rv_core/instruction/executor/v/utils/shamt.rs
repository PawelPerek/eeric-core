use crate::rv_core::vector_engine::SEW;

pub fn shamt(value: u64, sew: SEW) -> u64 {
    value & ((1 << sew.bit_length().ilog2()) - 1)
}

pub fn narrow_shamt(value: u64, sew: SEW) -> u64 {
    shamt(value, sew.double().unwrap())
}
