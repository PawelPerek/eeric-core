use crate::rv_core::{instruction::format::Csrr, registers::CsrRegisters};

pub fn csrrw(Csrr { rd, rs1, csr }: Csrr, c: &mut CsrRegisters) {
    todo!()
}
