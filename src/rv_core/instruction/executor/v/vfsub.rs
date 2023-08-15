use crate::rv_core::{
    instruction::format::{
        Opfvv,
        Opfvf
    }, 
    registers::{
        vector::VectorRegisters, 
        float::FloatRegisters
    }
};

pub fn vv(Opfvv { dest, vs1, vs2, vm }: Opfvv, v: &mut VectorRegisters) {
    v.apply(
        dest, 
        v
            .acquire_2(vs1, vs2)
            .map_fp(|(vel1, vel2)| vel1 - vel2)
    );
}

pub fn vf(Opfvf { vd, rs1, vs2, vm }: Opfvf, v: &mut VectorRegisters, f: &FloatRegisters) {
    v.apply(vd, v.acquire(vs2).map_fp(|vel| vel - f[rs1]));
}
