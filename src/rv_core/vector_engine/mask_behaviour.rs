// TODO: According to RVV spec undisturbed behaviour is correct in context of both undisturbed and agnostic.
// It will be nice QOL to implement one'ing unused mask and tail elements in the future.

#[derive(Clone, Default)]
pub enum MaskBehavior {
    #[default]
    Undisturbed,
    Agnostic,
}
