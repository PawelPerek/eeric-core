use crate::rv_core::instruction::executor::prelude::*;

pub fn v(vsx: Vsx, eew: SEW, v: &mut VectorContext<'_>, x: &IntegerRegisters, mem: &mut Memory) {
    super::vsox::v(vsx, eew, v, x, mem)
}
