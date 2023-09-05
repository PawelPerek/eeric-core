use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vlr { vd, rs1 }: Vlr,
    eew: SEW,
    nf: usize,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
    x: &IntegerRegisters,
    mem: &Memory,
) {
    let addr = x[rs1] as usize;
    let elements_amount = vec_engine.vlen.byte_length() / eew.byte_length();

    for segment in 0..nf {
        let mut vreg_data = Vec::new();

        for offset in 0..elements_amount {
            let address = addr
                .wrapping_add(offset.wrapping_mul(nf).wrapping_add(segment))
                .wrapping_mul(eew.byte_length());

            let data = match eew {
                SEW::E8 => u8::from_le_bytes(mem.get(address)) as u64,
                SEW::E16 => u16::from_le_bytes(mem.get(address)) as u64,
                SEW::E32 => u32::from_le_bytes(mem.get(address)) as u64,
                SEW::E64 => u64::from_le_bytes(mem.get(address)),
                _ => unimplemented!(),
            };

            vreg_data.push(data);
        }
        v.apply(
            vd + segment,
            vreg_data.into_iter().collect_with_eew(eew),
            &vec_engine,
        );
    }
}
