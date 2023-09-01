use crate::rv_core::{
    arbitrary_float::ArbitraryFloat, registers::vector::Vreg, vector_engine::SEW,
};

pub trait IterFPCollectorExt {
    fn collect_fp(self) -> Vreg;
}

impl<I> IterFPCollectorExt for I
where
    I: Iterator<Item = ArbitraryFloat>,
{
    fn collect_fp(self) -> Vreg {
        let mut eew = SEW::E8;

        let raw = self
            .flat_map(|fp| match fp {
                ArbitraryFloat::F32(f) => {
                    eew = SEW::E32;
                    f.to_le_bytes().to_vec()
                }
                ArbitraryFloat::F64(f) => {
                    eew = SEW::E64;
                    f.to_le_bytes().to_vec()
                }
            })
            .collect();

        Vreg { raw, eew }
    }
}
