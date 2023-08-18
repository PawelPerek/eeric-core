use crate::rv_core::{
    vector_engine::SEW,
    registers::vector::Vreg
};

pub trait IterEEWCollectorExt {
    fn collect_with_eew(self, eew: SEW) -> Vreg;
}

impl<I> IterEEWCollectorExt for I
    where
        I: Iterator<Item = u64>
{
    fn collect_with_eew(self, eew: SEW) -> Vreg {
        Vreg { 
            raw: self
                .map(u64::to_le_bytes)
                .flat_map(|bytes| bytes[0 .. eew.byte_length()].to_owned())
                .collect(), 
            eew 
        }
    }
}