use crate::rv_core::instruction::executor::prelude::*;

use super::utils::shamt::narrow_shamt;

pub fn wv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters, vec_engine: &VectorEngine) {
    let vreg = izip!(
        v.get_wide(vs2, vec_engine).iter_eew(),
        v.get(vs1, vec_engine).iter_eew()
    )
    .masked_map(
        v.default_mask(vm, vec_engine),
        v.get(vd, vec_engine).iter_eew(),
        |(vs2, vs1)| (vs2 >> narrow_shamt(vs1, vec_engine.sew.byte_length())) as u64,
    )
    .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn wx(
    Opivx { vd, rs1, vs2, vm }: Opivx,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
    x: &IntegerRegisters,
) {
    let vreg = v
        .get_wide(vs2, vec_engine)
        .iter_eew()
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get(vd, vec_engine).iter_eew(),
            |vs2| (vs2 >> narrow_shamt(x[rs1], vec_engine.sew.byte_length())) as u64,
        )
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn wi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters, vec_engine: &VectorEngine) {
    let vreg = v
        .get_wide(vs2, vec_engine)
        .iter_eew()
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get(vd, vec_engine).iter_eew(),
            |vs2| (vs2 >> narrow_shamt(imm5 as u64, vec_engine.sew.byte_length())) as u64,
        )
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
