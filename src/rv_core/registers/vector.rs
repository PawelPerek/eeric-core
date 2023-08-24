mod arbitrary_float;
mod vreg;
mod wide_vreg;

use std::collections::VecDeque;

use crate::rv_core::vector_engine::*;
pub use vreg::Vreg;
pub use wide_vreg::WideVreg;

#[derive(Clone)]
pub struct VectorRegisters {
    // Vector register holds 32 * VLENB bytes
    raw: Vec<u8>,
    pub vec_engine: VectorEngine,
}

impl VectorRegisters {
    fn new_zeros(vlen: VLEN, sew: SEW, lmul: LMUL) -> Self {
        Self {
            raw: vec![0x00; vlen.byte_length() * 32],
            vec_engine: VectorEngine::new(lmul, vlen, sew, Default::default(), Default::default())
                .unwrap(),
        }
    }

    fn start_ptr(&self, nth: usize) -> usize {
        nth * self.vec_engine.vlen.byte_length()
    }

    fn register_view(&self, nth: usize) -> impl Iterator<Item = u8> + '_ {
        let start = self.start_ptr(nth);

        // Note: Since we are working on multiples of two
        // multiplying 2^n (vlenb) by 2^(-n) (lmul) will not create floating point errors
        let end = start
            + (self.vec_engine.vlen.byte_length() as f32 * self.vec_engine.lmul.ratio()) as usize;

        self.raw[start..end].into_iter().copied()
    }

    pub fn get(&self, nth: usize) -> Vreg {
        Vreg::new(
            self.register_view(nth).collect(),
            self.vec_engine.sew.clone(),
        )
    }

    fn wide_register_view(&self, nth: usize) -> impl Iterator<Item = u8> + '_ {
        let start = self.start_ptr(nth);

        // Note: Since we are working on multiples of two
        // multiplying 2^n (vlenb) by 2^(-n) (lmul) will not create floating point errors
        let end = start
            + (self.vec_engine.vlen.byte_length() as f32 * self.vec_engine.lmul.double_ratio())
                as usize;

        self.raw[start..end].into_iter().copied()
    }

    pub fn get_wide(&self, nth: usize) -> WideVreg {
        WideVreg::new(
            self.wide_register_view(nth).collect(),
            self.vec_engine.sew.clone(),
        )
    }

    pub fn default_mask(&self, enabled: bool) -> MaskIterator {
        if enabled {
            MaskIterator::Exact(self.get(0).iter_mask().collect())
        } else {
            MaskIterator::Infinite(std::iter::repeat(1))
        }
    }

    pub fn apply(&mut self, nth: usize, vreg: Vreg) {
        let start = self.start_ptr(nth);
        let end = start
            + (self.vec_engine.vlen.byte_length() as f32 * self.vec_engine.lmul.ratio()) as usize;
        let vreg_length = end - start;

        self.raw[start..end].clone_from_slice(&vreg.raw[0..vreg_length])
    }

    // Useful when vreg holds less elements than VLEN / SEW (see vcompress.vm)
    pub fn partial_apply(&mut self, nth: usize, vreg: Vreg) {
        let start = self.start_ptr(nth);
        let vreg_length = vreg.raw.len();
        let end = start + vreg_length;

        self.raw[start..end].clone_from_slice(&vreg.raw[0..vreg_length])
    }
}

impl Default for VectorRegisters {
    fn default() -> Self {
        Self::new_zeros(VLEN::new_128(), SEW::new_8(), LMUL::M1)
    }
}

pub enum MaskIterator {
    Exact(VecDeque<u64>),
    Infinite(std::iter::Repeat<u64>),
}

impl Iterator for MaskIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Exact(vec) => vec.pop_front(),
            Self::Infinite(iter) => iter.next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn api() {
        let mut vregs = VectorRegisters::default();

        let vreg = izip!(vregs.get(0).iter_eew(), vregs.get(8).iter_eew())
            .map(|(rs1_el, rs2_el)| rs1_el + rs2_el)
            .collect_with_eew(vregs.vec_engine.sew.clone());

        vregs.apply(0, vreg);
    }
}
