use crate::rv_core::{
    instruction::format::Opfvf, 
    registers::{
        VectorRegisters, 
        FloatRegisters
    }
};

pub fn vf(Opfvf { vd, rs1, vs2, vm }: Opfvf, v: &mut VectorRegisters, f: &FloatRegisters) {
    v.apply(vd, v.acquire(vs2).map_fp(|vel| f[rs1] - vel));
}
