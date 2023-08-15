use crate::rv_core::{
    instruction::format::{
        Opivv,
        Opivx
    }, 
    registers::{
        VectorRegisters, 
        IntegerRegisters
    }
};

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters) {
    v.apply(vd, v.acquire_2(vs1, vs2).map(|(vel1, vel2)| if vel1 < vel2 { vel1 } else { vel2 }));
}

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    v.apply(vd, v.acquire(vs2).map(|vel| if vel < x[rs1] { vel } else { x[rs1] }));
}