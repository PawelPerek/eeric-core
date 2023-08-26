use crate::rv_core::{
    instruction::format::Csri,
    registers::{CsrRegisters, IntegerRegisters},
};

pub fn csrrwi(Csri { rd, uimm, csr }: Csri, x: &mut IntegerRegisters, c: &mut CsrRegisters) {
    let csr_value = c[csr];
    c[csr] = uimm as u64;
    x[rd] = csr_value;
}
