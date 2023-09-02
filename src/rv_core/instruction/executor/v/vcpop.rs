use crate::rv_core::instruction::executor::prelude::*;

pub fn m(
    Vwxunary0 {
        dest: rd, vs2, vm, ..
    }: Vwxunary0,
    v: &VectorRegisters, vec_engine: &VectorEngine,
    x: &mut IntegerRegisters,
) {
    let mask_count = izip!(v.default_mask(vm, vec_engine), v.get(vs2, vec_engine).iter_mask())
        .filter(|&(vs2_mask, v0_mask)| vs2_mask == 1 && v0_mask == 1)
        .count();

    x[rd] = mask_count as u64;
}
