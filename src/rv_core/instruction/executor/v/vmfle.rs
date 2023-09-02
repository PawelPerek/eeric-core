use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(Opfvv { dest: vd, vs1, vs2, vm }: Opfvv, v: &mut VectorRegisters, vec_engine: &VectorEngine) {
    let vreg = izip!(v.get(vs2, vec_engine).iter_fp(), v.get(vs1, vec_engine).iter_fp())
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |(vs2, vs1)| {
            if vs2 <= vs1 { 1 } else { 0 }
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

pub fn vf(Opfvf { vd, rs1, vs2, vm }: Opfvf, v: &mut VectorRegisters, vec_engine: &VectorEngine, f: &FloatRegisters) {
    let vreg = v.get(vs2, vec_engine).iter_fp()
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |vs2| {
            if vs2 <= ArbitraryFloat::copy_type(&vs2, f[rs1]) { 1 } else { 0 }
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}