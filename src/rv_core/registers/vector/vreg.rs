use crate::rv_core::vector_engine::SEW;

#[derive(Clone)]
// A wrapper over vector unit raw data
pub struct Vreg {
    pub raw: Vec<u8>,

    // There are instructions that double SEW independently on SEW value from vector unit
    pub eew: SEW,
}

impl Vreg {
    pub fn new(raw: Vec<u8>, eew: SEW) -> Vreg {
        Vreg { raw, eew }
    }

    pub fn iter_byte(&self) -> VregByteIterator<'_> {
        VregByteIterator { vreg: self, ptr: 0 }
    }

    pub fn iter_eew(&self) -> VregEEWIterator<'_> {
        VregEEWIterator {
            byte_iterator: self.iter_byte(),
            eew: self.eew.clone(),
        }
    }

    // TODO: Needed?

    pub fn iter_eew_mul_2(&self) -> VregEEWIterator<'_> {
        VregEEWIterator {
            byte_iterator: self.iter_byte(),
            eew: self.eew.clone().double(),
        }
    }

    pub fn iter_eew_div_2(&self) -> VregEEWIterator<'_> {
        VregEEWIterator {
            byte_iterator: self.iter_byte(),
            eew: self.eew.clone().half().unwrap(),
        }
    }

    pub fn iter_eew_div_4(&self) -> VregEEWIterator<'_> {
        VregEEWIterator {
            byte_iterator: self.iter_byte(),
            eew: self.eew.clone().fourth().unwrap(),
        }
    }

    pub fn iter_eew_div_8(&self) -> VregEEWIterator<'_> {
        VregEEWIterator {
            byte_iterator: self.iter_byte(),
            eew: self.eew.clone().eighth().unwrap(),
        }
    }

    pub fn iter_eew_e16(&self) -> VregEEWIterator<'_> {
        VregEEWIterator {
            byte_iterator: self.iter_byte(),
            eew: SEW::new_16(),
        }
    }

    pub fn iter_mask(&self) -> VregMaskIterator<'_> {
        VregMaskIterator {
            eew_iterator: self.iter_eew(),
        }
    }

    pub fn iter_u64(&self) -> VregU64Iterator<'_> {
        VregU64Iterator {
            byte_iterator: self.iter_byte(),
        }
    }

    pub fn iter_f32(&self) -> VregF32Iterator<'_> {
        VregF32Iterator {
            byte_iterator: self.iter_byte(),
        }
    }

    pub fn iter_f64(&self) -> VregF64Iterator<'_> {
        VregF64Iterator {
            byte_iterator: self.iter_byte(),
        }
    }
}

impl FromIterator<u8> for Vreg {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut raw = Vec::new();
        raw.extend(iter);

        Vreg {
            raw,
            eew: SEW::new_8(),
        }
    }
}

/// Iterators

// byte-by-byte

pub struct VregByteIterator<'a> {
    vreg: &'a Vreg,
    ptr: usize,
}

impl<'a> Iterator for VregByteIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.vreg.raw.get(self.ptr).copied();
        self.ptr += 1;
        element
    }
}

impl<'a> ExactSizeIterator for VregByteIterator<'a> {
    fn len(&self) -> usize {
        self.vreg.raw.len() - self.ptr
    }
}

// EEW:

// Iterator

pub struct VregEEWIterator<'a> {
    byte_iterator: VregByteIterator<'a>,
    eew: SEW,
}

impl<'a> Iterator for VregEEWIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.byte_iterator.len() <= 0 {
            return None;
        }

        let mut bytes = [0x00_u8; std::mem::size_of::<u64>()];

        for i in 0..self.eew.byte_length() {
            let byte = self
                .byte_iterator
                .next()
                .expect("VregEEWIterator finished early, EEW is not divisible by VLEN*EMUL?");
            bytes[i] = byte;
        }

        Some(u64::from_le_bytes(bytes))
    }
}

// mask (1u64 or 0u64)

pub struct VregMaskIterator<'a> {
    eew_iterator: VregEEWIterator<'a>,
}

impl<'a> Iterator for VregMaskIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // mask is encoded as least significant bit of each element
        self.eew_iterator
            .next()
            .map(|vel| if vel & 1 == 1 { 1 } else { 0 })
    }
}

// u64

pub struct VregU64Iterator<'a> {
    byte_iterator: VregByteIterator<'a>,
}

impl<'a> Iterator for VregU64Iterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.byte_iterator.next_chunk().map(u64::from_le_bytes).ok()
    }
}

// f32

pub struct VregF32Iterator<'a> {
    byte_iterator: VregByteIterator<'a>,
}

impl<'a> Iterator for VregF32Iterator<'a> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.byte_iterator.next_chunk().map(f32::from_le_bytes).ok()
    }
}

// f64

pub struct VregF64Iterator<'a> {
    byte_iterator: VregByteIterator<'a>,
}

impl<'a> Iterator for VregF64Iterator<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.byte_iterator.next_chunk().map(f64::from_le_bytes).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e8() {
        let vector_data = vec![0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01];

        let vreg = Vreg::new(vector_data, SEW::new_8());

        let mut iter = vreg.iter_eew();

        assert_eq!(iter.next(), Some(0xef));
        assert_eq!(iter.next(), Some(0xcd));
        assert_eq!(iter.next(), Some(0xab));
        assert_eq!(iter.next(), Some(0x89));
        assert_eq!(iter.next(), Some(0x67));
        assert_eq!(iter.next(), Some(0x45));
        assert_eq!(iter.next(), Some(0x23));
        assert_eq!(iter.next(), Some(0x01));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn e16() {
        let vector_data = vec![0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01];

        let vreg = Vreg::new(vector_data, SEW::new_16());

        let mut iter = vreg.iter_eew();

        assert_eq!(iter.next(), Some(0xcdef));
        assert_eq!(iter.next(), Some(0x89ab));
        assert_eq!(iter.next(), Some(0x4567));
        assert_eq!(iter.next(), Some(0x0123));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn e32() {
        let vector_data = vec![0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01];

        let vreg = Vreg::new(vector_data, SEW::new_32());

        let mut iter = vreg.iter_eew();

        assert_eq!(iter.next(), Some(0x89abcdef));
        assert_eq!(iter.next(), Some(0x01234567));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn e64() {
        let vector_data = vec![0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01];

        let vreg = Vreg::new(vector_data, SEW::new_64());

        let mut iter = vreg.iter_eew();

        assert_eq!(iter.next(), Some(0x0123456789abcdef));
        assert_eq!(iter.next(), None);
    }
}
