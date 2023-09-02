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

    // Only for widening instructions
    M16
}

impl LMUL {
    pub fn ratio(&self) -> f32 {
        match self {
            Self::MF8 => 0.125,
            Self::MF4 => 0.25,
            Self::MF2 => 0.5,
            Self::M1 => 1.,
            Self::M2 => 2.,
            Self::M4 => 4.,
            Self::M8 => 8.,
            Self::M16 => 16.,
        }
    }

    pub fn double(self) -> Result<Self, &'static str> {
        let doubled = match self {
            Self::MF8 => Self::MF4,
            Self::MF4 => Self::MF2,
            Self::MF2 => Self::M1,
            Self::M1 => Self::M2,
            Self::M2 => Self::M4,
            Self::M4 => Self::M8,
            Self::M8 => Self::M16,
            Self::M16 => return Err("Doubling LMUL=16 is not defined"),
        };

        Ok(doubled)
    }
}