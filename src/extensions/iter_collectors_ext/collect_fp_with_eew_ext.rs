use crate::rv_core::{registers::vector::{Vreg, ArbitraryFloat}, vector_engine::SEW};

pub trait IterFPCollectorExt {
    fn collect_fp(self) -> Vreg;
}

impl<I> IterFPCollectorExt for I
where
    I: Iterator<Item = ArbitraryFloat>,
{
    fn collect_fp(self) -> Vreg {
        let mut eew = SEW::new_8();

        let raw = self
            .flat_map(|fp| match fp {
                ArbitraryFloat::F32(f) => {
                    eew = SEW::new_32();
                    f.to_le_bytes().to_vec()
                },
                ArbitraryFloat::F64(f) => {
                    eew = SEW::new_64();
                    f.to_le_bytes().to_vec()
                },
            })
            .collect();

        Vreg {
            raw,
            eew
        }
    }
}