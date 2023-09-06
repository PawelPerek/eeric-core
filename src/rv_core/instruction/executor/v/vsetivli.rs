use crate::rv_core::instruction::executor::prelude::*;

pub fn vsetivli(
    Vsetivli { rd, uimm, vtypei }: Vsetivli,
    x: &mut IntegerRegisters,
    v: &mut VectorContext<'_>,
) {
    v.set_csr(VTYPE, vtypei as u64);
    v.set_csr(VL, uimm as u64);
    x[rd] = uimm as u64;
}
