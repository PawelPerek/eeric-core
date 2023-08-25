use crate::rv_core::{instruction::format::Csrr, registers::{CsrRegisters, IntegerRegisters}};

pub fn csrrw(Csrr { rd, rs1, csr }: Csrr, x: &mut IntegerRegisters, c: &mut CsrRegisters) {
    let csr_value = c[csr];
    c[csr] = x[rs1];
    x[rd] = csr_value;
}
