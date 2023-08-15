
pub enum FP {
    F32(f32),
    F64(f64)
}

impl FP {
    pub fn to_le_bytes(self) -> Vec<u8> {
        match self {
            FP::F32(a) => a.to_le_bytes().to_vec(),
            FP::F64(a) => a.to_le_bytes().to_vec()
        }
    }
}

impl std::ops::Add for FP {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FP::F32(a), FP::F32(b)) => FP::F32(a + b),
            (FP::F64(a), FP::F64(b)) => FP::F64(a + b),
            _ => panic!("Mismatched FP types")
        }
    }
}

impl std::ops::Add<f32> for FP {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        match self {
            FP::F32(a) => FP::F32(a + rhs),
            FP::F64(a) => FP::F64(a + rhs as f64),
            _ => panic!("Mismatched FP types")
        }
    }
}

impl std::ops::Add<f64> for FP {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        match self {
            FP::F32(a) => FP::F32(a + rhs as f32),
            FP::F64(a) => FP::F64(a + rhs),
            _ => panic!("Mismatched FP types")
        }
    }
}

impl std::ops::Mul for FP {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FP::F32(a), FP::F32(b)) => FP::F32(a * b),
            (FP::F64(a), FP::F64(b)) => FP::F64(a * b),
            _ => panic!("Mismatched FP types")
        }
    }
}

impl Into<f32> for FP {
    fn into(self) -> f32 {
        match self {
            FP::F32(a) => a,
            FP::F64(a) => a as f32
        }
    }
}

impl Into<f64> for FP {
    fn into(self) -> f64 {
        match self {
            FP::F32(a) => a as f64,
            FP::F64(a) => a
        }
    }
}