/// Vector length multiplier
#[derive(Default, Clone)]
pub enum LMUL {
    // Fractional multipliers
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

#[derive(Clone)]
pub enum SEW {
    E8,
    E16,
    E32,
    E64,
    // Only for widening instructions
    E128
}

impl TryFrom<usize> for SEW {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let sew = match value {
            8 => Self::E8,
            16 => Self::E16,
            32 => Self::E32,
            64 => Self::E64,
            _ => return Err("Incorrect SEW (should be 8b, 16b, 32b or 64b)")
        };

        Ok(sew)
    }
}

impl SEW {
    pub fn double(self) -> Result<Self, &'static str> {
        let doubled = match self {
            Self::E8 => Self::E16,
            Self::E16 => Self::E32,
            Self::E32 => Self::E64,
            Self::E64 => Self::E128,
            Self::E128 => return Err("Doubing 128-bit SEW is not defined")
        };

        Ok(doubled)
    }

    pub fn half(self) -> Result<Self, &'static str> {
        let halved = match self {
            Self::E128 => Self::E64,
            Self::E64 => Self::E32,
            Self::E32 => Self::E16,
            Self::E16 => Self::E8,
            Self::E8 => return Err("Halving 8-bit SEW is not defined"),
        };

        Ok(halved)
    }

    pub fn fourth(self) -> Result<Self, &'static str> {
        let fourthed = match self {
            Self::E128 => Self::E32,
            Self::E64 => Self::E16,
            Self::E32 => Self::E8,
            Self::E16 => return Err("Fourthing 16-bit SEW is not defined"),
            Self::E8 => return Err("Fourthing 8-bit SEW is not defined"),
        };

        Ok(fourthed)
    }

    pub fn eighth(self) -> Result<Self, &'static str> {
        let eighted = match self {
            Self::E128 => Self::E16,
            Self::E64 => Self::E8,
            Self::E32 => return Err("Eighting 32-bit SEW is not defined"),
            Self::E16 => return Err("Eighting 16-bit SEW is not defined"),
            Self::E8 => return Err("Eighting 8-bit SEW is not defined"),
        };

        Ok(eighted)
    }

    pub fn bit_length(&self) -> usize {
        match self {
            Self::E8 => 8,
            Self::E16 => 16,
            Self::E32 => 32,
            Self::E64 => 64,
            Self::E128 => 128
        }
    }

    pub fn byte_length(&self) -> usize {
        self.bit_length() / 8
    }
}

impl Default for SEW {
    fn default() -> Self {
        Self::E8
    }
}

// TODO: According to RVV spec undisturbed behaviour is correct in context of both undisturbed and agnostic.
// It will be nice QOL to implement one'ing unused mask and tail elements in the future.

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
    #[allow(dead_code)]
    tail_elements: MaskBehavior,
    #[allow(dead_code)]
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
        if sew.bit_length() <= vlen.bit_length() {
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
