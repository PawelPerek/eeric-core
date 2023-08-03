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
    
            padded_bytes[(8-len)..].copy_from_slice(&self.raw[span]);
    
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
        let mut vector_data = vec![0x76, 0x54, 0x32, 0x10];

        let mut vreg = Vreg::new(vector_data, SEW::new(8).unwrap());

        assert_eq!(vreg.next(), Some(0x76));
        assert_eq!(vreg.next(), Some(0x54));
        assert_eq!(vreg.next(), Some(0x32));
        assert_eq!(vreg.next(), Some(0x10));
        assert_eq!(vreg.next(), None);
    }

    #[test]
    fn e16() {
        let mut vector_data = vec![0x76, 0x54, 0x32, 0x10];

        let mut vreg = Vreg::new(vector_data, SEW::new(16).unwrap());

        assert_eq!(vreg.next(), Some(0x5476));
        assert_eq!(vreg.next(), Some(0x1032));
        assert_eq!(vreg.next(), None);
    }

    #[test]
    fn e32() {
        let mut vector_data = vec![0x76, 0x54, 0x32, 0x10];

        let mut vreg = Vreg::new(vector_data, SEW::new(8).unwrap());

        assert_eq!(vreg.next(), Some(0x10325476));
        assert_eq!(vreg.next(), None);
    }
}