/// Vector unit size of microarchitecture
#[derive(Clone, Default)]
#[cfg_attr(debug_assertions, derive(Debug, PartialEq))]
pub enum VLEN {
    V64,
    #[default]
    V128,
    V256,
    V512
}

impl VLEN {
    pub fn bit_length(&self) -> usize {
        match self {
            Self::V64 => 64,
            Self::V128 => 128,
            Self::V256 => 256,
            Self::V512 => 512
        }
    }

    pub fn byte_length(&self) -> usize {
        self.bit_length() / 8
    }
}