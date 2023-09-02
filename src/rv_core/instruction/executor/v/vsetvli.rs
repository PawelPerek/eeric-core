use crate::rv_core::instruction::executor::prelude::*;

pub fn vsetvli(
    Vsetvli { rd, rs1, vtypei }: Vsetvli,
    x: &mut IntegerRegisters,
    v: &VectorRegisters, vec_engine: &VectorEngine,
    csr: &mut CsrRegisters,
) {
    todo!()
}
