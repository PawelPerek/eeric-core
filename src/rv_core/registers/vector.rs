mod vreg;
mod wide_vreg;

use std::collections::VecDeque;

use crate::prelude::*;
pub use vreg::Vreg;
pub use wide_vreg::WideVreg;

#[derive(Clone, Default, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct VectorRegisters(Vec<u8>);

impl Snapshotable for VectorRegisters {
    type Snapshot = Vec<u8>;

    fn snapshot(&self) -> Self::Snapshot {
        self.0.clone()
    }
}

impl VectorRegisters {
    pub fn default(vec_engine: &VectorEngine) -> Self {
        let vlenb = vec_engine.vlen.byte_length();

        Self(vec![0x00; vlenb * 32])
    }

    fn start_ptr(&self, nth: usize, vec_engine: &VectorEngine) -> usize {
        nth * vec_engine.vlen.byte_length()
    }

    fn register_view(&self, nth: usize, vec_engine: &VectorEngine) -> impl Iterator<Item = u8> + '_ {
        let start = self.start_ptr(nth, vec_engine);

        // Note: Since we are working on multiples of two
        // multiplying 2^n (vlenb) by 2^(-n) (lmul) will not create floating point errors
        let end = start
            + (vec_engine.vlen.byte_length() as f32 * vec_engine.lmul.ratio()) as usize;

        self.0[start..end].into_iter().copied()
    }

    pub fn get(&self, nth: usize, vec_engine: &VectorEngine) -> Vreg {
        Vreg::new(
            self.register_view(nth, vec_engine).collect(),
            vec_engine.sew.clone(),
        )
    }

    fn wide_register_view(&self, nth: usize, vec_engine: &VectorEngine) -> impl Iterator<Item = u8> + '_ {
        let start = self.start_ptr(nth, vec_engine);

        // Note: Since we are working on multiples of two
        // multiplying 2^n (vlenb) by 2^(-n) (lmul) will not create floating point errors
        let end = start
            + (vec_engine.vlen.byte_length() as f32 * vec_engine.lmul.clone().double().unwrap().ratio())
                as usize;

        self.0[start..end].into_iter().copied()
    }

    pub fn get_wide(&self, nth: usize, vec_engine: &VectorEngine) -> WideVreg {
        WideVreg::new(
            self.wide_register_view(nth, vec_engine).collect(),
            vec_engine.sew.clone(),
        )
    }

    pub fn default_mask(&self, enabled: bool, vec_engine: &VectorEngine) -> MaskIterator {
        if enabled {
            MaskIterator::Exact(self.get(0, vec_engine).iter_mask().collect())
        } else {
            MaskIterator::Infinite(std::iter::repeat(1))
        }
    }

    pub fn apply(&mut self, nth: usize, vreg: Vreg, vec_engine: &VectorEngine) {
        let engine_vlen =
            (vec_engine.vlen.byte_length() as f32 * vec_engine.lmul.ratio()) as usize;
        let start = self.start_ptr(nth, vec_engine);

        if vreg.iter_byte().len() >= engine_vlen {
            let end = start + engine_vlen;
            let vreg_length = end - start;

            self.0[start..end].clone_from_slice(&vreg.raw[0..vreg_length])
        } else {
            // Vreg has fractional EMUL or has less elements than VLEN / SEW
            let vreg_length = vreg.raw.len();
            let end = start + vreg_length;

            self.0[start..end].clone_from_slice(&vreg.raw[0..vreg_length])
        }
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
