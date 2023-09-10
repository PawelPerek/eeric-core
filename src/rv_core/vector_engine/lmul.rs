/// Vector length multiplier
#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
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
            Self::MF8 => 0.125,
            Self::MF4 => 0.25,
            Self::MF2 => 0.5,
            Self::M1 => 1.,
            Self::M2 => 2.,
            Self::M4 => 4.,
            Self::M8 => 8.,
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
            Self::M8 => return Err("Doubling LMUL=8 is reserved"),
        };

        Ok(doubled)
    }
}

impl TryFrom<f32> for LMUL {
    type Error = String;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        let lmul = if value == 0.125 {
            LMUL::MF8
        } else if value == 0.25 {
            LMUL::MF4
        } else if value == 0.5 {
            LMUL::MF2
        } else if value == 1.0 {
            LMUL::M1
        } else if value == 2.0 {
            LMUL::M2
        } else if value == 4.0 {
            LMUL::M4
        } else if value == 8.0 {
            LMUL::M8
        } else {
            return Err(format!("Cannot parse {} into LMUL", value));
        };

        Ok(lmul)
    }
}
