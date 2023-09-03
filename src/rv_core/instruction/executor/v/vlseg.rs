use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vl { vd, rs1, vm }: Vl,
    eew: SEW,
    nf: usize,
    v: &mut VectorRegisters,
    vec_engine: &VectorEngine,
    x: &IntegerRegisters,
    mem: &Memory,
) {
    let addr = x[rs1] as usize;
    let element_amount = vec_engine.vlen.bit_length() / vec_engine.sew.bit_length();

    for segment in 0..nf {
        let mut vn = Vec::new();
        
        for offset in 0..element_amount {
            let address = addr
                .wrapping_add(
                    offset
                        .wrapping_mul(nf)
                        .wrapping_add(segment)
                )
                .wrapping_mul(eew.byte_length());

            let element: u64 = match eew {
                SEW::E8 => u8::from_le_bytes(mem.get(address)) as u64,
                SEW::E16 => u16::from_le_bytes(mem.get(address)) as u64,
                SEW::E32 => u32::from_le_bytes(mem.get(address)) as u64,
                SEW::E64 => u64::from_le_bytes(mem.get(address)),
                _ => unimplemented!(),
            };

            vn.push(element);
        }
        
        let vreg = v
        .get(vd + segment, vec_engine)
        .iter_eew()
        .enumerate()
        .masked_map(
            v.default_mask(vm, vec_engine),
            v.get(vd + segment, vec_engine).iter_eew(),
            |(index, _)| vn[index],
        )
        .collect_with_eew(vec_engine.sew.clone());
        v.apply(vd + segment, vreg, vec_engine);
    };
}

#[cfg(test)]
mod tests {
    use crate::rv_core::vector_engine::VectorEngineBuilder;

    use super::*;

    #[test]
    fn test_vlseg_rgb() {
        let mem_content = vec![
            1, 2, 3,
            4, 5, 6, 
            7, 8, 9,
            10, 11, 12,
            13, 14, 15,
            16, 17, 18,
            19, 20, 21,
            22, 23, 24,
            25, 26, 27
        ];
        let mem = Memory::from(mem_content);

        let mut x = IntegerRegisters::default();
        x[1] = 0;

        let vec_engine = VectorEngineBuilder::default()
            .vlen(VLEN::V64)
            .build()
            .unwrap();
        let mut v = VectorRegisters::default(&vec_engine);
        let eew = SEW::E8;
        let nf = 3;

        super::v(Vl { vd: 0, rs1: 1, vm: false }, eew, nf, &mut v, &vec_engine, &x, &mem);

        let red = v.get(0, &vec_engine).iter_eew().collect_vec();
        let green = v.get(1, &vec_engine).iter_eew().collect_vec();
        let blue = v.get(2, &vec_engine).iter_eew().collect_vec();

        assert_eq!(red, vec![1, 4, 7, 10, 13, 16, 19, 22]);
        assert_eq!(green, vec![2, 5, 8, 11, 14, 17, 20, 23]);
        assert_eq!(blue, vec![3, 6, 9, 12, 15, 18, 21, 24]);
    }

    #[test]
    fn test_vlseg_complex() {
        let mem_content = vec![
            1, 0, 0, 0, 2, 0, 0, 0,
            3, 0, 0, 0, 4, 0, 0, 0,
            5, 0, 0, 0, 6, 0, 0, 0 
        ];
        let mem = Memory::from(mem_content);

        let mut x = IntegerRegisters::default();
        x[1] = 0;

        let vec_engine = VectorEngineBuilder::default()
            .vlen(VLEN::V64)
            .sew(SEW::E32)
            .build()
            .unwrap();
        let mut v = VectorRegisters::default(&vec_engine);
        let eew = SEW::E32;
        let nf = 2;

        super::v(Vl { vd: 0, rs1: 1, vm: false }, eew, nf, &mut v, &vec_engine, &x, &mem);

        let real = v.get(0, &vec_engine).iter_eew().collect_vec();
        let complex = v.get(1, &vec_engine).iter_eew().collect_vec();

        assert_eq!(real, vec![1, 3]);
        assert_eq!(complex, vec![2, 4]);
    }
}
