use crate::prelude::*;

use crate::rv_core::{instruction::format::Csrr, registers::{CsrRegisters, IntegerRegisters}};

pub fn csrrs(Csrr { rd, rs1, csr }: Csrr, x: &mut IntegerRegisters, c: &mut CsrRegisters) {
    let csr_value = c[csr];
    x[rd] = csr_value;
    
    if rs1 != ZERO {
        let clear_mask = x[rs1];
        c[csr] = csr_value | clear_mask;
    }
}
