use crate::rv_core::instruction::executor::prelude::*;

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, vec_engine: &VectorEngine, x: &IntegerRegisters) {
    let offset = x[rs1] as usize;

    let vs2_snapshot = v.get(vs2, vec_engine).iter_eew().collect_vec();

    let vreg = v
        .get(vd, vec_engine)
        .iter_eew()
        .enumerate()
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |(index, _)| {
            vs2_snapshot.get(index + offset).copied().unwrap_or(0)
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn vi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters, vec_engine: &VectorEngine) {
    let offset = imm5 as usize;

    let vs2_snapshot = v.get(vs2, vec_engine).iter_eew().collect_vec();

    let vreg = v
        .get(vd, vec_engine)
        .iter_eew()
        .enumerate()
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |(index, _)| {
            vs2_snapshot.get(index + offset).copied().unwrap_or(0)
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
