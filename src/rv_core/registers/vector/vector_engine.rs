use std::{cell::RefCell, rc::Rc};

/// Vector length multiplier
#[derive(Default, Clone)]
pub enum LMUL {
    // Fractional multipliers, not supported yet
    // TODO: support fractional multipliers
    MF8,
    MF4,
    MF2,

    // Integer multipliers
    #[default]
    M1,
    M2,
    M4,
    M8,
}

/// Vector unit size of microarchitecture
#[derive(Clone)]
pub struct VLEN(usize);

impl VLEN {
    fn new(length: usize) -> Result<Self, &'static str> {
        // VLEN=32 is the smallest VLEN required (Zvl32b)
        if length >= 32 && length.count_ones() == 1 {
            Ok(Self(length))
        } else {
            Err("Length of VLEN must be greater or equal 32 and a power of two")
        }
    }
}

impl Default for VLEN {
    fn default() -> Self {
        Self::new(128).unwrap()
    }
}

/// Size of an element inside vector
// RISC-V vector extension spec v1.0 defines four SEW lengths:
// (000 - 8b, 001 - 16b, 010 - 32b, 011 - 64b).
#[derive(Clone)]
pub struct SEW(usize);

impl SEW {
    pub fn new(length: usize) -> Result<Self, &'static str> {
        if length <= 64 && length >= 8 && length.count_ones() == 1 {
            Ok(Self(length))
        } else {
            Err("Length of SEW must be one of the 8, 16, 32, 64")
        }
    }

    pub fn new_8() -> Self {
        Self::new(8).unwrap()
    }

    pub fn new_16() -> Self {
        Self::new(16).unwrap()
    }

    pub fn new_32() -> Self {
        Self::new(32).unwrap()
    }

    pub fn new_64() -> Self {
        Self::new(64).unwrap()
    }

    pub fn bit_length(&self) -> usize {
        self.0
    }

    pub fn byte_length(&self) -> usize {
        self.0 / 8
    } 
}

impl Default for SEW {
    fn default() -> Self {
        Self::new(8).unwrap()
    }
}

#[derive(Clone, Default)]
pub enum MaskBehavior {
    #[default]
    Undisturbed,
    Agnostic,
}

#[derive(Clone, Default)]
pub struct VectorEngine {
    lmul: LMUL,
    vlen: VLEN,
    pub sew: SEW,
    tail_elements: MaskBehavior,
    inactive_elements: MaskBehavior,
}

impl VectorEngine {
    pub fn new(
        lmul: LMUL,
        vlen: VLEN,
        sew: SEW,
        tail_elements: MaskBehavior,
        inactive_elements: MaskBehavior,
    ) -> Result<Self, &'static str> {
        if sew.0 <= vlen.0 {
            Ok(Self {
                lmul,
                vlen,
                sew,
                tail_elements,
                inactive_elements
            })
        } else {
            Err("SEW can't be longer than VLEN")
        }
    }

    pub fn vlen(&self) -> usize {
        self.vlen.0 as usize
    }

    pub fn sew(&self) -> usize {
        self.sew.0 as usize
    }

    pub fn vlenb(&self) -> usize {
        self.vlen() / 8
    }

    pub fn lmul(&self) -> f32 {
        match self.lmul {
            LMUL::MF8 => 0.125,
            LMUL::MF4 => 0.25,
            LMUL::MF2 => 0.5,
            LMUL::M1 => 1.,
            LMUL::M2 => 2.,
            LMUL::M4 => 4.,
            LMUL::M8 => 8.,
        }
    }

    pub fn vlmax(&self) -> usize {
        ((self.vlen() / self.sew()) as f32 * self.lmul() as f32) as usize
    }
}
