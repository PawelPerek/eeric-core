use crate::rv_core::instruction::executor::prelude::*;
use crate::rv_core::instruction::executor::prelude::*;

pub fn vx(
    Opmvx {
        dest: vd,
        rs1,
        vs2,
        vm,
    }: Opmvx,
    v: &mut VectorRegisters,
    x: &IntegerRegisters,
) {
    let last_value = u64::to_le_bytes(x[rs1]);

    let vreg_values: Vreg = v
        .get(vs2)
        .iter_byte()
        .take(v.vec_engine.vlmax() - v.vec_engine.sew.byte_length())
        .chain(
            last_value[0..v.vec_engine.sew.byte_length()]
                .into_iter()
                .copied(),
        )
        .collect();

    let vreg = vreg_values
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vd| vd)
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
