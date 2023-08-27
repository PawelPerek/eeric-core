use crate::rv_core::instruction::executor::prelude::*;

pub fn csrrc(Csrr { rd, rs1, csr }: Csrr, x: &mut IntegerRegisters, c: &mut CsrRegisters) {
    let csr_value = c[csr];
    x[rd] = csr_value;

    if rs1 != ZERO {
        let clear_mask = x[rs1];
        c[csr] = csr_value & !clear_mask;
    }
}
