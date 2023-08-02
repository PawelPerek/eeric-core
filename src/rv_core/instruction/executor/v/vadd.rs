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

pub fn vv(Opivv { vd, vs1, vs2 }: Opivv, v: &mut VectorRegisters) {
    v[vd] = v.acquire2(vs1, vs2).execute(|vel1, vel2| vel1 + vel2);
}
pub fn vx(Opivx { vd, rs1, vs2 }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    v[vd] = v.acquire(vs2).execute(|vel| vel + x[rs1]);
}
pub fn vi(Opivi { vd, imm5, vs2 }: Opivi, v: &mut VectorRegisters) {
    v[vd] = v.acquire(vs2).execute(|vel| vel + imm5);
}