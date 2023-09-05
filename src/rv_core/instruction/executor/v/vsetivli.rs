use crate::rv_core::instruction::executor::prelude::*;

pub fn vsetivli(
    Vsetivli { rd, uimm, vtypei }: Vsetivli,
    x: &mut IntegerRegisters,
    v: &mut VectorContext<'_>,
) {
    todo!()
}
