use crate::rv_core::{
    instruction::format::Opfvf, 
    registers::{
        VectorRegisters, 
        FloatRegisters
    }
};

pub fn vf(Opfvf { vd, rs1, vs2, vm }: Opfvf, v: &mut VectorRegisters, f: &FloatRegisters) {
    todo!()
}
