use crate::rv_core::{
    instruction::format::Csri, 
    registers::CsrRegisters
};

pub fn csrrci(Csri { rd, uimm, csr }: Csri, c: &mut CsrRegisters) {
   todo!()
}