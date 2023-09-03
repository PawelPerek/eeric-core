use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vlx { vd, rs1, vs2, vm }: Vlx,
    eew: SEW,
    nf: usize,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
    x: &IntegerRegisters,
    mem: &Memory,
) {
    let addr = x[rs1] as usize;
    let vs2 = v.get(vs2, vec_engine).iter_custom_eew(eew).collect_vec();

    let element_amount = vec_engine.vlen.bit_length() / vec_engine.sew.bit_length();

    for segment in 0..nf {
        let mut store = Vec::with_capacity(element_amount);

        for offset in 0..element_amount {
            let offset = vs2[offset] as usize;
            let address = addr.wrapping_add(offset).wrapping_add(segment);

            let element: u64 = match vec_engine.sew {
                SEW::E8 => u8::from_le_bytes(mem.get(address)) as u64,
                SEW::E16 => u16::from_le_bytes(mem.get(address)) as u64,
                SEW::E32 => u32::from_le_bytes(mem.get(address)) as u64,
                SEW::E64 => u64::from_le_bytes(mem.get(address)),
                _ => unimplemented!(),
            };

            store.push(element);
        }

        let vreg = v
            .get(vd + segment, vec_engine)
            .iter_eew()
            .enumerate()
            .masked_map(
                v.default_mask(vm, vec_engine),
                v.get(vd + segment, vec_engine).iter_eew(),
                |(index, _)| store[index],
            )
            .collect_with_eew(vec_engine.sew.clone());

        v.apply(vd + segment, vreg, vec_engine);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vloxseg_basic() {
        let mem_content = (0..100).collect_vec();

        let mem = Memory::from(mem_content);
        let mut x = IntegerRegisters::default();
        let vec_engine = VectorEngineBuilder::default()
            .vlen(VLEN::V64)
            .build()
            .unwrap();
        let mut v = VectorRegisters::default(&vec_engine);

        x[5] = 0;  // base address

        v.apply(2, vec![0, 5, 2, 1, 4, 8, 12, 22].into_iter().collect(), &vec_engine);

        super::v(Vlx { vd: 0, rs1: 5, vs2: 2, vm: false }, SEW::E8, 2, &mut v, &vec_engine, &x, &mem);

        let first_segment = v.get(0, &vec_engine).iter_eew().collect_vec();
        let second_segment = v.get(1, &vec_engine).iter_eew().collect_vec();

        assert_eq!(first_segment, vec![0, 5, 2, 1, 4, 8, 12, 22]);
        assert_eq!(second_segment, vec![1, 6, 3, 2, 5, 9, 13, 23]);
    }
}