use crate::rv_core::instruction::executor::prelude::*;

pub fn vsetvli(
    Vsetvli { rd, rs1, vtypei }: Vsetvli,
    x: &mut IntegerRegisters,
    v: &mut VectorContext<'_>,
) {
    v.set_csr(VTYPE, vtypei as u64);

    let avl = match (rd, rs1) {
        (ZERO, ZERO) => v.csr[VL],
        (_, ZERO) => v.vlmax() as u64,
        (_, rs1) => x[rs1].min(v.vlmax() as u64)
    };

    v.set_csr(VL, avl);
    x[rd] = avl;
}
