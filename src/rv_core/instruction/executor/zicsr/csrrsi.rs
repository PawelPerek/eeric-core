use crate::rv_core::{
    instruction::format::Csri,
    registers::{CsrRegisters, IntegerRegisters},
};

pub fn csrrsi(Csri { rd, uimm, csr }: Csri, x: &mut IntegerRegisters, c: &mut CsrRegisters) {
    let csr_value = c[csr];
    x[rd] = csr_value;

    let clear_mask = uimm as u64;
    c[csr] = csr_value | clear_mask;
}
