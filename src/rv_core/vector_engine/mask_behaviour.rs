// TODO: According to RVV spec undisturbed behaviour is correct in context of both undisturbed and agnostic.
// It will be nice QOL to implement one'ing unused mask and tail elements in the future.

#[derive(Clone, Default)]
#[cfg_attr(debug_assertions, derive(Debug, PartialEq))]
pub enum MaskBehavior {
    #[default]
    Undisturbed,
    Agnostic,
}
