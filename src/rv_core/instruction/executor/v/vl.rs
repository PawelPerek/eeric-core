use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vl { vd, rs1, vm }: Vl,
    eew: SEW,
    x: &IntegerRegisters,
    v: &mut VectorContext<'_>,
    mem: &Memory,
) {
    let addr = x[rs1] as usize;

    let element_amount = v.vec_engine.vlen.bit_length() / v.vec_engine.sew.bit_length();

    let mut store = Vec::<u64>::with_capacity(element_amount);

    for offset in 0..element_amount {
        let element: u64 = match eew.byte_length() {
            1 => u8::from_le_bytes(mem.get(addr + offset)) as u64,
            2 => u16::from_le_bytes(mem.get(addr + offset * 2)) as u64,
            4 => u32::from_le_bytes(mem.get(addr + offset * 4)) as u64,
            8 => u64::from_le_bytes(mem.get(addr + offset * 8)),
            _ => unimplemented!(),
        };

        store.push(element);
    }

    let vreg = v
        .get(vd)
        .iter_eew()
        .enumerate()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |(index, _)| {
            store[index]
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
