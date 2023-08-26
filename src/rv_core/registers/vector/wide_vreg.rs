use crate::rv_core::{vector_engine::SEW, ArbitraryFloat};

#[derive(Clone)]
pub struct WideVreg {
    pub raw: Vec<u8>,
    pub eew: SEW,
}

impl WideVreg {
    pub fn new(raw: Vec<u8>, eew: SEW) -> Self {
        Self {
            raw,
            eew: eew.double(),
        }
    }

    pub fn iter_byte(&self) -> WideVregByteIterator<'_> {
        WideVregByteIterator { vreg: self, ptr: 0 }
    }

    pub fn iter_eew(&self) -> WideVregEEWIterator<'_> {
        WideVregEEWIterator {
            byte_iterator: self.iter_byte(),
            eew: self.eew.clone(),
        }
    }

    pub fn iter_fp(&self) -> WideVregFPIterator<'_> {
        WideVregFPIterator {
            byte_iterator: self.iter_byte(),
            eew: self.eew.clone(),
        }
    }
}

impl FromIterator<u8> for WideVreg {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut raw = Vec::new();
        raw.extend(iter);

        WideVreg {
            raw,
            eew: SEW::new_8(),
        }
    }
}

/// Iterators

// byte-by-byte

pub struct WideVregByteIterator<'a> {
    vreg: &'a WideVreg,
    ptr: usize,
}

impl<'a> Iterator for WideVregByteIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.vreg.raw.get(self.ptr).copied();
        self.ptr += 1;
        element
    }
}

impl<'a> ExactSizeIterator for WideVregByteIterator<'a> {
    fn len(&self) -> usize {
        self.vreg.raw.len() - self.ptr
    }
}

// EEW

pub struct WideVregEEWIterator<'a> {
    byte_iterator: WideVregByteIterator<'a>,
    eew: SEW,
}

impl<'a> Iterator for WideVregEEWIterator<'a> {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        if self.byte_iterator.len() <= 0 {
            return None;
        }

        let mut bytes = [0x00_u8; std::mem::size_of::<u128>()];

        for i in 0..self.eew.byte_length() {
            let byte = self
                .byte_iterator
                .next()
                .expect("WideVregEEWIterator finished early, EEW is not divisible by VLEN*EMUL?");
            bytes[i] = byte;
        }

        Some(u128::from_le_bytes(bytes))
    }
}

// Float EEW

pub struct WideVregFPIterator<'a> {
    byte_iterator: WideVregByteIterator<'a>,
    eew: SEW,
}

// Note: yeah, 32b SEW is only supported byte length for wide vreg in RVV 1.0 :/

impl<'a> Iterator for WideVregFPIterator<'a> {
    type Item = ArbitraryFloat;

    fn next(&mut self) -> Option<Self::Item> {
        match self.eew.byte_length() {
            4 => self
                .byte_iterator
                .next_chunk()
                .map(f64::from_le_bytes)
                .map(ArbitraryFloat::F64)
                .ok(),
            _ => panic!("Invalid SEW for wide floating point"),
        }
    }
}
