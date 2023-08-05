use crate::rv_core::{
    instruction::format::vector::{
        Opivv,
        Opivx,
        Opivi,
    }, 
    registers::{
        vector::VectorRegisters, 
        integer::IntegerRegisters
    }
};

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    v.apply(vd, v.acquire(vs2).execute(|vel| x[rs1] - vel));
}
pub fn vi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters) {
    v.apply(vd, v.acquire(vs2).execute(|vel| imm5 - vel));
}