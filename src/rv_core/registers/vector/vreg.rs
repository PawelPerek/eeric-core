use std::{cell::RefCell, rc::Rc};

use super::vector_engine::{SEW, LMUL, VLEN};

#[derive(Clone)]
// A wrapper over vector unit raw data
pub struct Vreg { 
    pub raw: Vec<u8>,

    // There are instructions that double SEW independently on SEW value from vector unit
    pub sew: SEW,
    ptr: usize
}

impl Vreg {
    pub fn new<'ve>(raw: Vec<u8>, sew: SEW) -> Vreg {
        Vreg { raw, sew, ptr: 0 }
    } 

    pub fn double_sew(self) -> Vreg {
        Vreg { raw: self.raw, ptr: self.ptr, sew: SEW::new(self.sew.bit_length() * 2).unwrap()}
    }
}

impl Iterator for Vreg {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let byte_step: usize = self.sew.byte_length();
        let span = self.ptr .. self.ptr + byte_step;
        
        if span.end <= self.raw.len() {
            let mut padded_bytes = [0; 8];
            let len = self.raw[span.clone()].len();
            padded_bytes[..len].copy_from_slice(&self.raw[span]);
    
            self.ptr += byte_step;

            Some(u64::from_le_bytes(padded_bytes))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e8() {
        let vector_data = vec![0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01];

        let mut vreg = Vreg::new(vector_data, SEW::new(8).unwrap());


        assert_eq!(vreg.next(), Some(0xef));
        assert_eq!(vreg.next(), Some(0xcd));
        assert_eq!(vreg.next(), Some(0xab));
        assert_eq!(vreg.next(), Some(0x89));
        assert_eq!(vreg.next(), Some(0x67));
        assert_eq!(vreg.next(), Some(0x45));
        assert_eq!(vreg.next(), Some(0x23));
        assert_eq!(vreg.next(), Some(0x01));
        assert_eq!(vreg.next(), None);
    }

    #[test]
    fn e16() {
        let vector_data = vec![0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01];

        let mut vreg = Vreg::new(vector_data, SEW::new(16).unwrap());


        assert_eq!(vreg.next(), Some(0xcdef));
        assert_eq!(vreg.next(), Some(0x89ab));
        assert_eq!(vreg.next(), Some(0x4567));
        assert_eq!(vreg.next(), Some(0x0123));
        assert_eq!(vreg.next(), None);
    }

    #[test]
    fn e32() {
        let vector_data = vec![0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01];

        let mut vreg = Vreg::new(vector_data, SEW::new(32).unwrap());


        assert_eq!(vreg.next(), Some(0x89abcdef));
        assert_eq!(vreg.next(), Some(0x01234567));
        assert_eq!(vreg.next(), None);
    }

    #[test]
    fn e64() {
        let vector_data = vec![0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01];

        let mut vreg = Vreg::new(vector_data, SEW::new(64).unwrap());


        assert_eq!(vreg.next(), Some(0x0123456789abcdef));
        assert_eq!(vreg.next(), None);
    }
}