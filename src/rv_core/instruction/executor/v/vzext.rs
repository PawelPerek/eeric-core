use crate::rv_core::instruction::executor::prelude::*;

pub fn vf2(
    Vxunary0 {
        dest: vd, vs2, vm, ..
    }: Vxunary0,
    v: &mut VectorContext<'_>,
) {
    let vreg = v
        .get(vs2)
        .iter_eew_div_2()
        .take(v.vlmax() / 2)
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| vs2)
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn vf4(
    Vxunary0 {
        dest: vd, vs2, vm, ..
    }: Vxunary0,
    v: &mut VectorContext<'_>,
) {
    let vreg = v
        .get(vs2)
        .iter_eew_div_4()
        .take(v.vlmax() / 4)
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| vs2)
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}

pub fn vf8(
    Vxunary0 {
        dest: vd, vs2, vm, ..
    }: Vxunary0,
    v: &mut VectorContext<'_>,
) {
    let vreg = v
        .get(vs2)
        .iter_eew_div_8()
        .take(v.vlmax() / 8)
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| vs2)
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
