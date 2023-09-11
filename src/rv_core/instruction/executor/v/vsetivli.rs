use crate::rv_core::instruction::executor::prelude::*;

pub fn vsetivli(
    Vsetivli { rd, uimm, vtypei }: Vsetivli,
    x: &mut IntegerRegisters,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    v.set_vtype(vtypei as u64).unwrap();
    v.csr[VL].write(uimm as u64)?;
    x[rd] = uimm as u64;

    Ok(())
}
