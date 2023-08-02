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
pub struct VLEN(u16);

impl VLEN {
    fn new(length: u16) -> Result<Self, &'static str> {
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
pub struct SEW(u16);

impl SEW {
    pub fn new(length: u16) -> Result<Self, &'static str> {
        if length <= 64 && length >= 8 && length.count_ones() == 1 {
            Ok(Self(length))
        } else {
            Err("Length of SEW must be one of the 8, 16, 32, 64")
        }
    }

    pub fn byte_length(&self) -> usize {
        (self.0 / 8).into()
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

#[derive(Clone)]
pub struct VectorEngine {
    lmul: LMUL,
    vlen: VLEN,
    sew: SEW,
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
        use LMUL::*;

        match self.lmul {
            MF8 => 0.125,
            MF4 => 0.25,
            MF2 => 0.5,
            M1 => 1.,
            M2 => 2.,
            M4 => 4.,
            M8 => 8.,
        }
    }
}
