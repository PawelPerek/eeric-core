use crate::rv_core::instruction::executor::prelude::*;

pub fn v(Vl { vd, rs1, vm: _ }: Vl, v: &mut VectorContext<'_>, x: &IntegerRegisters, mem: &Memory) {
    let addr = x[rs1] as usize;

    let element_amount = v.vec_engine.vlen.byte_length();

    let store = (0..element_amount)
        .map(|offset| addr.wrapping_add(offset))
        .map(|address| mem.get(address))
        .map(u8::from_le_bytes)
        .collect_vec();

    let vreg = v
        .get(vd)
        .iter_byte()
        .enumerate()
        .map(|(index, _)| {
            store[index]
        })
        .collect();

    v.apply(vd, vreg);
}
