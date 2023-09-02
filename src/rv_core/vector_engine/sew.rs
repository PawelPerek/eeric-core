#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum SEW {
    #[default]
    E8,
    E16,
    E32,
    E64,
    // Only for widening instructions
    E128,
}

impl TryFrom<usize> for SEW {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let sew = match value {
            8 => Self::E8,
            16 => Self::E16,
            32 => Self::E32,
            64 => Self::E64,
            _ => return Err("Incorrect SEW (should be 8b, 16b, 32b or 64b)"),
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
            Self::E128 => return Err("Doubing 128-bit SEW is not defined"),
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
            Self::E128 => 128,
        }
    }

    pub fn byte_length(&self) -> usize {
        self.bit_length() / 8
    }
}
