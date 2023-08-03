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

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters) {
    v.apply(vd, v.acquire(vs1).execute(|vel| vel));
}
pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    v.apply(vd, v.acquire(vd).execute(|_| x[rs1]));
}
pub fn vi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters) {
    v.apply(vd, v.acquire(vd).execute(|_| imm5));
}