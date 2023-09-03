mod lmul;
mod mask_behaviour;
mod sew;
mod vlen;

use derive_builder::Builder;
pub use lmul::LMUL;
pub use mask_behaviour::MaskBehavior;
pub use sew::SEW;
pub use vlen::VLEN;

use super::snapshot::Snapshotable;

#[derive(Builder, Clone, Copy, Default, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct VectorEngine {
    #[builder(default)]
    pub lmul: LMUL,
    #[builder(default)]
    pub vlen: VLEN,
    #[builder(default)]
    pub sew: SEW,
    #[builder(default)]
    #[allow(dead_code)]
    pub tail_elements: MaskBehavior,
    #[builder(default)]
    #[allow(dead_code)]
    pub inactive_elements: MaskBehavior,
}

impl VectorEngine {
    pub fn new(
        lmul: LMUL,
        vlen: VLEN,
        sew: SEW,
        tail_elements: MaskBehavior,
        inactive_elements: MaskBehavior,
    ) -> Self {
        Self {
            lmul,
            vlen,
            sew,
            tail_elements,
            inactive_elements,
        }
    }

    pub fn vlmax(&self) -> usize {
        ((self.vlen.bit_length() / self.sew.bit_length()) as f32 * self.lmul.ratio()) as usize
    }
}

impl Snapshotable for VectorEngine {
    type Snapshot = Self;

    fn snapshot(&self) -> Self::Snapshot {
        Self {
            lmul: self.lmul.clone(),
            vlen: self.vlen.clone(),
            sew: self.sew.clone(),
            tail_elements: self.tail_elements.clone(),
            inactive_elements: self.inactive_elements.clone(),
        }
    }
}
