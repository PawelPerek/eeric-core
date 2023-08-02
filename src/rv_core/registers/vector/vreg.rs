use std::{cell::RefCell, rc::Rc};

use super::vector_engine::{SEW, LMUL, VLEN};


// A wrapper over vector unit raw data
pub struct Vreg<'ve> { 
    pub raw: &'ve mut [u8],
    sew: Rc<RefCell<SEW>>
}

impl Vreg<'_> {
    pub fn new<'ve>(raw: &'ve mut [u8], sew: Rc<RefCell<SEW>>) -> Vreg<'ve> {
        Vreg { raw, sew }
    } 

    fn first(&self) -> u64 {
        let span = 0 .. self.sew.borrow().byte_length();
        
        let mut padded_bytes = [0; 8];
        let len = self.raw[span.clone()].len();

        padded_bytes[(8-len)..].copy_from_slice(&self.raw[span]);

        u64::from_le_bytes(padded_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first() {
        let mut vector_data = vec![0x76, 0x54, 0x32, 0x10];

        let vreg = Vreg {
            raw: &mut vector_data[..],
            sew: &SEW::new(16).unwrap(),
        };

        assert_eq!(vreg.first(), 0x5476);
    }
}