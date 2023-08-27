use crate::rv_core::instruction::executor::prelude::*;
    
    


pub fn v(
    Opivi {
        vd,
        imm5: _,
        vs2,
        vm,
    }: Opivi,
    v: &mut VectorRegisters,
) {
    for i in 0..1 {
        // Note: vs1 = vs2
        super::vmv::vv(
            Opivv {
                vd: vd + i,
                vs1: vs2 + i,
                vs2: 0,
                vm,
            },
            v,
        )
    }
}
