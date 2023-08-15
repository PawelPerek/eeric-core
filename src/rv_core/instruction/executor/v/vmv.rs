use crate::rv_core::{
    instruction::format::{
        Opivv,
        Opivx,
        Opivi,
        Vwxunary0,
        Vrxunary0
    }, 
    registers::{
        VectorRegisters, 
        IntegerRegisters
    }
};

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorRegisters) {
    v.apply(vd, v.acquire(vs1).map(|vel| vel));
}

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    v.apply(vd, v.acquire(vd).map(|_| x[rs1]));
}

pub fn vi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters) {
    v.apply(vd, v.acquire(vd).map(|_| imm5));
}


pub fn xs(Vwxunary0 { dest, vs2, vm, .. }: Vwxunary0, v: &mut VectorRegisters) {
    todo!()
}

pub fn sx(Vrxunary0 { dest, vs1, vm, .. }: Vrxunary0, v: &mut VectorRegisters) {
    todo!()
}
