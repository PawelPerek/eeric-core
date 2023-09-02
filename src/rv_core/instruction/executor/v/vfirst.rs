use crate::rv_core::instruction::executor::prelude::*;

pub fn m(
    Vwxunary0 {
        dest: rd, vs2, vm, ..
    }: Vwxunary0,
    v: &VectorRegisters,
    vec_engine: &VectorEngine,
    x: &mut IntegerRegisters,
) {
    let maybe_index = izip!(
        v.default_mask(vm, vec_engine),
        v.get(vs2, vec_engine).iter_mask()
    )
    .enumerate()
    .find(|&(_, (v0_mask, vs2_mask))| v0_mask == 1 && vs2_mask == 1)
    .map(|(index, _)| index as u64)
    .unwrap_or(u64::MAX);

    x[rd] = maybe_index;
}
