mod lmul;
mod sew;
mod vlen;
mod mask_behaviour;

pub use lmul::LMUL;
pub use sew::SEW;
pub use vlen::VLEN;
pub use mask_behaviour::MaskBehavior;

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

    pub fn snapshot(&self) -> VectorEngineSnapshot {
        VectorEngineSnapshot {
            lmul: self.lmul.clone(),
            vlen: self.vlen.clone(),
            sew: self.sew.clone(),
            tail_elements: self.tail_elements.clone(),
            inactive_elements: self.inactive_elements.clone(),
        }
    } 
}

#[derive(Clone)]

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct VectorEngineSnapshot {
    pub lmul: LMUL,
    pub vlen: VLEN,
    pub sew: SEW,
    pub tail_elements: MaskBehavior,
    pub inactive_elements: MaskBehavior,
}