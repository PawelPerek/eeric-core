pub fn shamt(value: u64, sew: usize) -> u64 {
    value & ((1 << sew.ilog2()) - 1)
}

pub fn narrow_shamt(value: u64, sew: usize) -> u64 {
    shamt(value, sew * 2)
}
