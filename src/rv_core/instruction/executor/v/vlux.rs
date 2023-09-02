use crate::rv_core::instruction::executor::prelude::*;

pub fn v(Vlx { vd, rs1, vs2, vm }: Vlx, eew: SEW, v: &mut VectorRegisters, vec_engine: &VectorEngine, x: &IntegerRegisters, mem: &Memory) {
    super::vlox::v(Vlx { vd, rs1, vs2, vm }, eew, v, vec_engine, x, mem)
}
