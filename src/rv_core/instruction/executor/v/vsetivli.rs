use crate::rv_core::instruction::executor::prelude::*;

pub fn vsetivli(
    Vsetivli { rd, uimm, vtypei }: Vsetivli,
    x: &mut IntegerRegisters,
    v: &VectorRegisters,
    vec_engine: &VectorEngine,
    csr: &mut CsrRegisters,
) {
    todo!()
}
