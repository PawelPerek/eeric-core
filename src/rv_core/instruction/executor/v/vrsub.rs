use crate::rv_core::instruction::executor::prelude::*;

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, vec_engine: &VectorEngine, x: &IntegerRegisters) {
    let vreg = v
        .get(vs2, vec_engine)
        .iter_eew()
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |vs2| {
            x[rs1].wrapping_sub(vs2)
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn vi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters, vec_engine: &VectorEngine) {
    let vreg = v
        .get(vs2, vec_engine)
        .iter_eew()
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |vs2| {
            (imm5 as u64).wrapping_sub(vs2)
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
