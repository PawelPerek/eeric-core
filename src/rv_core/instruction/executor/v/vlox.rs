use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vlx { vd, rs1, vs2, vm }: Vlx,
    eew: SEW,
    v: &mut VectorContext<'_>,
    x: &IntegerRegisters,
    mem: &Memory,
) {
    let addr = x[rs1] as usize;
    let vs2 = v.get(vs2).iter_custom_eew(eew).collect_vec();

    let element_amount = v.vec_engine.vlen.bit_length() / v.vec_engine.sew.bit_length();

    let mut store = Vec::with_capacity(element_amount);

    for offset in 0..element_amount {
        let offset = vs2[offset] as usize;
        let address = addr.wrapping_add(offset);

        let element: u64 = match v.vec_engine.sew.byte_length() {
            1 => u8::from_le_bytes(mem.get(address)) as u64,
            2 => u16::from_le_bytes(mem.get(address)) as u64,
            4 => u32::from_le_bytes(mem.get(address)) as u64,
            8 => u64::from_le_bytes(mem.get(address)),
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
        .collect_with_eew(v.vec_engine.sew.clone());

    v.apply(vd, vreg);
}
