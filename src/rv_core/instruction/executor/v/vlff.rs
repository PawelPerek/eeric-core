use crate::rv_core::{instruction::executor::prelude::*, registers::aliases::csr};

pub fn v(
    Vl { vd, rs1, vm }: Vl,
    eew: SEW,
    v: &mut VectorRegisters, vec_engine: &VectorEngine,
    x: &IntegerRegisters,
    c: &mut CsrRegisters,
    mem: &Memory,
) {
    let addr = x[rs1] as usize;

    let element_amount = vec_engine.vlen.bit_length() / vec_engine.sew.bit_length();

    let mut store = Vec::<u64>::with_capacity(element_amount);

    for offset in 0..element_amount {
        let result = match eew.byte_length() {
            1 => mem
                .fallible_get(addr + offset)
                .map(u8::from_le_bytes)
                .map(Into::into),
            2 => mem
                .fallible_get(addr + offset * 2)
                .map(u16::from_le_bytes)
                .map(Into::into),
            4 => mem
                .fallible_get(addr + offset * 4)
                .map(u32::from_le_bytes)
                .map(Into::into),
            8 => mem.fallible_get(addr + offset * 8).map(u64::from_le_bytes),
            _ => unimplemented!(),
        };

        match result {
            Some(element) => store.push(element),
            None => {
                if offset == 0 {
                    unimplemented!() // trap not implemented
                } else {
                    c[csr::VL] = offset as u64;
                }
            }
        };
    }

    let vreg = v
        .get(vd, vec_engine)
        .iter_eew()
        .enumerate()
        .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |(index, _)| {
            store[index]
        })
        .collect_with_eew(vec_engine.sew.clone());

    v.apply(vd, vreg, vec_engine)
}
