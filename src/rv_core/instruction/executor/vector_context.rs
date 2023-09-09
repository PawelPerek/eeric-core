use std::collections::VecDeque;

use crate::rv_core::{
    registers::{CsrRegisters, VectorRegisters},
    vector_engine::VectorEngine,
};

use super::prelude::{
    aliases::csr::{VL, VTYPE},
    vector::{Vreg, WideVreg},
    LMUL, SEW,
};

pub struct VectorContext<'c> {
    pub v: &'c mut VectorRegisters,
    pub csr: &'c mut CsrRegisters,
    pub vec_engine: &'c mut VectorEngine,
}

impl VectorContext<'_> {
    fn start_ptr(&self, nth: usize) -> usize {
        nth * self.vec_engine.vlen.byte_length()
    }

    fn register_view_with_lmul(&self, nth: usize, lmul: LMUL) -> impl Iterator<Item = u8> + '_ {
        let start = self.start_ptr(nth);

        // Note: Since we are working on multiples of two
        // multiplying 2^n (vlenb) by 2^(±n) (lmul) will not create floating point errors
        let vlmax = (self.vec_engine.vlen.byte_length() as f32 * lmul.ratio()) as usize;

        let vl = self.csr[VL];

        self.v.0[start..start + vlmax.min(vl as usize)]
            .iter()
            .copied()
    }

    fn register_view(&self, nth: usize) -> impl Iterator<Item = u8> + '_ {
        self.register_view_with_lmul(nth, self.vec_engine.lmul)
    }

    pub fn get(&self, nth: usize) -> Vreg {
        Vreg::new(self.register_view(nth).collect(), self.vec_engine.sew)
    }

    fn wide_register_view(&self, nth: usize) -> impl Iterator<Item = u8> + '_ {
        self.register_view_with_lmul(nth, self.vec_engine.lmul.double().unwrap())
    }

    pub fn get_wide(&self, nth: usize) -> WideVreg {
        WideVreg::new(self.wide_register_view(nth).collect(), self.vec_engine.sew)
    }

    fn single_register_view(&self, nth: usize) -> impl Iterator<Item = u8> + '_ {
        self.register_view_with_lmul(nth, LMUL::M1)
    }

    pub fn get_single(&self, nth: usize) -> Vreg {
        Vreg::new(
            self.single_register_view(nth).collect(),
            self.vec_engine.sew,
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

            self.v.0[start..end].clone_from_slice(&vreg.raw[0..vreg_length])
        } else {
            // Vreg has fractional EMUL or has less elements than VLEN / SEW
            let vreg_length = vreg.raw.len();
            let end = start + vreg_length;

            self.v.0[start..end].clone_from_slice(&vreg.raw[0..vreg_length])
        }
    }

    pub fn vlmax(&self) -> usize {
        ((self.vec_engine.vlen.bit_length() / self.vec_engine.sew.bit_length()) as f32
            * self.vec_engine.lmul.ratio()) as usize
    }

    pub fn set_csr(&mut self, csr: usize, value: u64) {
        self.csr[csr] = value;
    }

    fn decompose_vtype(vtype: u64) -> Result<RawVType, String> {
        let vlmul = (vtype & 0b111) as u8;
        let vsew = ((vtype >> 3) & 0b111) as u8;
        let vta = (vtype >> 6) & 0b1 == 1;
        let vma = (vtype >> 7) & 0b1 == 1;

        let reserved = (vtype << 1) >> 9;
        if reserved != 0 {
            return Err(format!(
                "vtype[XLEN-2:8] other than 0 is reserved, got {}",
                reserved
            ));
        }

        Ok(RawVType {
            vlmul,
            vsew,
            vta,
            vma,
        })
    }

    pub fn set_vtype(&mut self, value: u64) -> Result<(), String> {
        self.csr[VTYPE] = value;

        let raw_vtype = Self::decompose_vtype(value)?;

        self.vec_engine.lmul = match raw_vtype.vlmul {
            0b100 => return Err(String::from("vlmul=100 is reserved")),
            0b101 => LMUL::MF8,
            0b110 => LMUL::MF4,
            0b111 => LMUL::MF2,
            0b000 => LMUL::M1,
            0b001 => LMUL::M2,
            0b010 => LMUL::M4,
            0b011 => LMUL::M8,
            _ => unreachable!(),
        };

        self.vec_engine.sew = match raw_vtype.vsew {
            0b000 => SEW::E8,
            0b001 => SEW::E16,
            0b010 => SEW::E32,
            0b011 => SEW::E64,
            0b100..=0b111 => return Err(String::from("vsew=1xx is reserved")),
            _ => unreachable!(),
        };

        use super::prelude::MaskBehavior::*;
        self.vec_engine.tail_elements = if raw_vtype.vta { Agnostic } else { Undisturbed };
        self.vec_engine.inactive_elements = if raw_vtype.vma { Agnostic } else { Undisturbed };

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
struct RawVType {
    pub vlmul: u8,
    pub vsew: u8,
    pub vta: bool,
    pub vma: bool,
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
    use super::{RawVType, VectorContext};

    #[test]
    fn vtype_parsing() {
        let vtype = 193;

        let raw_vtype = VectorContext::decompose_vtype(vtype);

        assert_eq!(
            raw_vtype,
            Ok(RawVType {
                vlmul: 0b001,
                vsew: 0b000,
                vta: true,
                vma: true
            })
        )
    }
}
