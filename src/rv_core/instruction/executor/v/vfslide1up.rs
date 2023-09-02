use crate::rv_core::instruction::executor::prelude::*;

pub fn vf(Opfvf { vd, rs1, vs2, vm }: Opfvf, v: &mut VectorRegisters, vec_engine: &VectorEngine, f: &FloatRegisters) {
    let first_value = f64::to_le_bytes(f[rs1]);
    let vreg_values: Vreg = first_value[0..vec_engine.sew.byte_length()]
        .into_iter()
        .copied()
        .chain(v.get(vs2, vec_engine).iter_byte().skip(vec_engine.sew.byte_length()))
        .take(vec_engine.vlmax())
        .collect();

    let vreg = vreg_values
        .iter_eew()
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |vd| vd)
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
