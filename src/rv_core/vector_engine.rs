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

impl LMUL {
    pub fn ratio(&self) -> f32 {
        match self {
            LMUL::MF8 => 0.125,
            LMUL::MF4 => 0.25,
            LMUL::MF2 => 0.5,
            LMUL::M1 => 1.,
            LMUL::M2 => 2.,
            LMUL::M4 => 4.,
            LMUL::M8 => 8.,
        }
    }

    pub fn double_ratio(&self) -> f32 {
        self.ratio() * 2.0
    }
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

    pub fn new_128() -> Self {
        Self::new(128).unwrap()
    }

    pub fn new_256() -> Self {
        Self::new(256).unwrap()
    }

    pub fn new_512() -> Self {
        Self::new(512).unwrap()
    }

    pub fn bit_length(&self) -> usize {
        self.0
    }

    pub fn byte_length(&self) -> usize {
        self.0 / 8
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
    fn new(length: usize) -> Result<Self, &'static str> {
        if length <= 128 && length >= 8 && length.count_ones() == 1 {
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

    pub fn double(self) -> Self {
        Self(self.0 * 2)
    }

    pub fn half(self) -> Result<Self, &'static str> {
        Self::new(self.0 / 2)
    }

    pub fn fourth(self) -> Result<Self, &'static str> {
        Self::new(self.0 / 4)
    }

    pub fn eighth(self) -> Result<Self, &'static str> {
        Self::new(self.0 / 8)
    }

    pub fn bit_length(&self) -> usize {
        self.0
    }

    pub fn byte_length(&self) -> usize {
        self.bit_length() / 8
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
    pub lmul: LMUL,
    pub vlen: VLEN,
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
                inactive_elements,
            })
        } else {
            Err("SEW can't be longer than VLEN")
        }
    }

    pub fn vlmax(&self) -> usize {
        ((self.vlen.bit_length() / self.sew.bit_length()) as f32 * self.lmul.ratio()) as usize
    }
}
