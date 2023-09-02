use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters, vec_engine: &VectorEngine) {
    let vreg = izip!(v.get(vs2, vec_engine).iter_eew(), v.get(vs1, vec_engine).iter_eew())
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |(vs2, vs1)| {
            if (vs2 as i64) > (vs1 as i64) {
                vs2
            } else {
                vs1
            }
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, vec_engine: &VectorEngine, x: &IntegerRegisters) {
    let vreg = v
        .get(vs2, vec_engine)
        .iter_eew()
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |vs2| {
            if (vs2 as i64) > (x[rs1] as i64) {
                vs2
            } else {
                x[rs1]
            }
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
