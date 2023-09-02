use crate::rv_core::instruction::executor::prelude::*;

pub fn vsetvl(
    Vsetvl { rd, rs1, rs2 }: Vsetvl,
    x: &mut IntegerRegisters,
    v: &VectorRegisters,
    vec_engine: &VectorEngine,
    csr: &mut CsrRegisters,
) {
    todo!()
}
