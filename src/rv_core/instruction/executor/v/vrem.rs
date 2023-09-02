use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(
    Opmvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opmvv,
    v: &mut VectorRegisters, vec_engine: &VectorEngine,
) {
    let vreg = izip!(v.get(vs2, vec_engine).iter_eew(), v.get(vs1, vec_engine).iter_eew())
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get(vd, vec_engine).iter_eew(),
            |(dividend, divisor)| {
                if divisor == 0 {
                    dividend
                } else {
                    ((dividend as i64) % (divisor as i64)) as u64
                }
            },
        )
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}

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
    let vreg = v
        .get(vs2, vec_engine)
        .iter_eew()
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |dividend| {
            let divisor = x[rs1];

            if divisor == 0 {
                dividend
            } else {
                ((dividend as i64) % (divisor as i64)) as u64
            }
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine);
}
