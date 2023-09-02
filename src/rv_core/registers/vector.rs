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
    pub fn snapshot(&self) -> [Vec<u8>; 32] {
        let mut snapshot = [0; 32].map(|_| Vec::new());

        for i in 0..32 {
            let vreg = self.get(i);
            snapshot[i] = vreg.iter_byte().collect();
        }

        snapshot
    }

    fn new_zeros(vlen: VLEN, sew: SEW, lmul: LMUL) -> Self {
        Self {
            raw: vec![0x00; vlen.byte_length() * 32],
            vec_engine: VectorEngine::new(lmul, vlen, sew, Default::default(), Default::default())
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
            + (self.vec_engine.vlen.byte_length() as f32 * self.vec_engine.lmul.clone().double().unwrap().ratio())
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
        let engine_vlen =
            (self.vec_engine.vlen.byte_length() as f32 * self.vec_engine.lmul.ratio()) as usize;
        let start = self.start_ptr(nth);

        if vreg.iter_byte().len() >= engine_vlen {
            let end = start + engine_vlen;
            let vreg_length = end - start;

            self.raw[start..end].clone_from_slice(&vreg.raw[0..vreg_length])
        } else {
            // Vreg has fractional EMUL or has less elements than VLEN / SEW
            let vreg_length = vreg.raw.len();
            let end = start + vreg_length;

            self.raw[start..end].clone_from_slice(&vreg.raw[0..vreg_length])
        }
    }
}

impl Default for VectorRegisters {
    fn default() -> Self {
        Self::new_zeros(Default::default(), Default::default(), Default::default())
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
