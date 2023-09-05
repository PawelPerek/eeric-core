use crate::rv_core::instruction::executor::prelude::*;

pub fn vsetvl(
    Vsetvl { rd, rs1, rs2 }: Vsetvl,
    x: &mut IntegerRegisters,
    v: &mut VectorContext<'_>,
) {
    todo!()
}
