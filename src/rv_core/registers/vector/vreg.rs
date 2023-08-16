use super::vector_engine::SEW;

#[derive(Clone)]
// A wrapper over vector unit raw data
pub struct Vreg { 
    pub raw: Vec<u8>,

    // There are instructions that double SEW independently on SEW value from vector unit
    pub eew: SEW,
    ptr: usize
}

impl Vreg {
    pub fn new(raw: Vec<u8>, eew: SEW) -> Vreg {
        Vreg { raw, eew, ptr: 0 }
    } 

    pub fn double_sew(self) -> Vreg {
        Vreg { raw: self.raw, ptr: self.ptr, eew: SEW::new(self.eew.bit_length() * 2).unwrap()}
    }

    pub fn byte_length(&self) -> usize {
        self.len() * self.eew.byte_length()
    }

    pub fn iter_eew(&mut self) -> VregEEWIterator<'_> {
        VregEEWIterator { vreg: self, ptr: 0 }
    }

    pub fn iter_u64(&mut self) -> VregU64Iterator<'_> {
        VregU64Iterator { vreg: self }
    }

    pub fn iter_f32(&mut self) -> VregF32Iterator<'_> {
        VregF32Iterator { vreg: self }
    } 

    pub fn iter_f64(&mut self) -> VregF64Iterator<'_> {
        VregF64Iterator { vreg: self }
    } 
}

impl Iterator for Vreg {
    type Item = u8;
    
    fn next(&mut self) -> Option<Self::Item> {
        let element = self.raw.get(self.ptr).copied();
        self.ptr += 1;
        element
    }
}

impl FromIterator<u8> for Vreg {
    fn from_iter<T: IntoIterator<Item=u8>>(iter: T) -> Self {
        let mut raw = Vec::new();
        raw.extend(iter);

        Vreg { raw, ptr: 0, eew: SEW::new(8).unwrap() }
    }
}

pub struct VregEEWIterator<'a> {
    vreg: &'a mut Vreg,
    ptr: usize
}

impl<'a> Iterator for VregEEWIterator<'a> {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr + self.vreg.eew.byte_length() > self.vreg.raw.len() {
            return None;
        }
        
        let mut bytes = [0x00_u8; 8];

        let chunk_range = self.ptr .. self.ptr + self.vreg.eew.byte_length();
        let chunk = &self.vreg.raw[chunk_range];
        
        bytes[..chunk.len()].copy_from_slice(chunk);

        Some(u64::from_le_bytes(bytes))
    }
}

pub struct VregU64Iterator<'a> {
    vreg: &'a mut Vreg
}

impl<'a> Iterator for VregU64Iterator<'a> {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.vreg.next_chunk().map(u64::from_le_bytes).ok()
    }
}

pub struct VregF32Iterator<'a> {
    vreg: &'a mut Vreg
}

impl<'a> Iterator for VregF32Iterator<'a> {
    type Item = f32;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.vreg.next_chunk().map(f32::from_le_bytes).ok()
    }
}

pub struct VregF64Iterator<'a> {
    vreg: &'a mut Vreg
}

impl<'a> Iterator for VregF64Iterator<'a> {
    type Item = f64;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.vreg.next_chunk().map(f64::from_le_bytes).ok()
    }
}

impl ExactSizeIterator for Vreg {
    fn len(&self) -> usize {
        self.raw.len() / self.eew.byte_length()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e8() {
        let vector_data = vec![0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01];

        let mut vreg = Vreg::new(vector_data, SEW::new(8).unwrap());

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

        let mut vreg = Vreg::new(vector_data, SEW::new(16).unwrap());

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

        let mut vreg = Vreg::new(vector_data, SEW::new(32).unwrap());

        let mut iter = vreg.iter_eew();

        assert_eq!(iter.next(), Some(0x89abcdef));
        assert_eq!(iter.next(), Some(0x01234567));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn e64() {
        let vector_data = vec![0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01];

        let mut vreg = Vreg::new(vector_data, SEW::new(64).unwrap());

        let mut iter = vreg.iter_eew();

        assert_eq!(iter.next(), Some(0x0123456789abcdef));
        assert_eq!(iter.next(), None);
    }
}