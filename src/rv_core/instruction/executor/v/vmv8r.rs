use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Opivi {
        vd,
        imm5: _,
        vs2,
        vm,
    }: Opivi,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
) {
    for i in 0..8 {
        super::vmv::vv(
            Opivv {
                vd: vd + i,
                vs1: vs2 + i,
                vs2: 0,
                vm,
            },
            v,
            vec_engine,
        )
    }
}
