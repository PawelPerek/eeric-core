use super::{Vreg, fp::FP};

/// 1 register

pub struct AcquiredRegister {
    vs: Vreg,
}

impl AcquiredRegister {
    pub fn map<F>(&self, builder: F) -> Vreg
    where
        F: Fn(u64) -> u64,
    {
        self.vs.clone().iter_u64().map(builder).flat_map(u64::to_le_bytes).collect()
    }

    pub fn map_fp<Builder>(&self, builder: Builder) -> Vreg
    where
        Builder: Fn(FP) -> FP,
    {
        match self.vs.eew.byte_length() {
            4 => self.vs.clone().iter_f32().map(FP::F32).map(builder).flat_map(FP::to_le_bytes).collect(),
            8 => self.vs.clone().iter_f64().map(FP::F64).map(builder).flat_map(FP::to_le_bytes).collect(),
            _ => panic!("Floating point vector operations in RVV v1.0 are only supported for 32 and 64 bit floats")
        }
    }
}

/// 2 Regsiters

pub struct Acquired2Registers {
    vs1: Vreg,
    vs2: Vreg,
}

impl Acquired2Registers {
    pub fn map<F>(&self, builder: F) -> Vreg
    where
        F: Fn((u64, u64)) -> u64,
    {
        self.vs1.clone().iter_u64().zip(self.vs2.clone().iter_u64()).map(builder).flat_map(u64::to_le_bytes).collect()
    }

    pub fn map_fp<Builder>(&self, builder: Builder) -> Vreg
    where
        Builder: Fn((FP, FP)) -> FP,
    {
        match self.vs1.eew.byte_length() {
            4 => self.vs1.clone().iter_f32().map(FP::F32).zip(
                self.vs2.clone().iter_f32().map(FP::F32)
            ).map(builder).flat_map(FP::to_le_bytes).collect(),
            8 => self.vs1.clone().iter_f64().map(FP::F64).zip(
                self.vs2.clone().iter_f64().map(FP::F64)
            ).map(builder).flat_map(FP::to_le_bytes).collect(),
            _ => panic!("Floating point vector operations in RVV v1.0 are only supported for 32 and 64 bit floats")
        }
    }
}

/// 2 Registers and mask

fn kinda_curry<T>(x: ((T, T), T)) -> (T, T, T) {
    let ((a, b), c) = x;
    (a, b, c)
}

pub struct Acquired2RegistersWithMask {
    vs1: Vreg,
    vs2: Vreg,
    vm: Option<Vreg>,
}

impl Acquired2RegistersWithMask {
    pub fn map<F>(&self, builder: F) -> Vreg
    where
        F: Fn((u64, u64, u64)) -> u64,
    {
        self.vs1.clone()
            .iter_u64()
            .zip(self.vs2.clone().iter_u64())
            .zip(self.vm.clone().unwrap_or(Vreg::new(
                vec![0x00; self.vs1.byte_length()],
                self.vs1.eew.clone(),
            )).iter_u64())
            .map(kinda_curry)
            .map(builder)
            .flat_map(u64::to_le_bytes).collect()
    }
}
