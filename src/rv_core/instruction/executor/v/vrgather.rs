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
    let vlmax = v.vec_engine.vlmax();

    v.apply(vd, v.acquire(vs1).map(|index| {
        if index as usize >= vlmax { 
            0 
        } else { 
            v.get(vs2).iter_eew().nth(index as usize).unwrap()
        }
    }));
}

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorRegisters, x: &IntegerRegisters) {
    let vlmax = v.vec_engine.vlmax();
    let index = x[rs1] as usize;
    
    v.apply(vd, if index >= vlmax { 
        v.acquire(vd).map(|_| 0) 
    } else {
        v.acquire(vd).map(|_| v.get(vs2).iter_eew().nth(index).unwrap())
    } );
}

pub fn vi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorRegisters) {
    let vlmax = v.vec_engine.vlmax();
    let index = imm5 as usize;

    v.apply(vd, if index >= vlmax { 
        v.acquire(vd).map(|_| 0) 
    } else {
        v.acquire(vd).map(|_| v.get(vs2).iter_eew().nth(index).unwrap())
    } );
}