use crate::rv_core::instruction::executor::prelude::*;

pub fn vx(
    Opmvx {
        dest: vd,
        rs1,
        vs2,
        vm,
    }: Opmvx,
    v: &mut VectorRegisters, vec_engine: &VectorEngine,
    x: &IntegerRegisters,
) {
    let last_value = u64::to_le_bytes(x[rs1]);

    let vreg_values: Vreg = v
        .get(vs2, vec_engine)
        .iter_byte()
        .take(vec_engine.vlmax() - vec_engine.sew.byte_length())
        .chain(
            last_value[0..vec_engine.sew.byte_length()]
                .into_iter()
                .copied(),
        )
        .collect();

    let vreg = vreg_values
        .iter_eew()
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |vd| vd)
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
