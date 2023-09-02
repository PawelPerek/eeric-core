use crate::rv_core::instruction::executor::prelude::*;

pub fn v(Vl { vd, rs1, vm }: Vl, eew: SEW, nf: usize, v: &mut VectorRegisters, vec_engine: &VectorEngine, x: &IntegerRegisters, mem: &Memory) {
    let addr = x[rs1] as usize;

    let element_amount = vec_engine.sew.byte_length() / vec_engine.sew.bit_length();

    let mut store: Vec<Vec<u64>> = vec![vec![0; element_amount]; nf];

    for offset in 0..element_amount {
        for segment in 0..nf {
            let element: u64 = match eew.byte_length() {
                1 => u8::from_le_bytes(mem.get(addr + offset)) as u64,
                2 => u16::from_le_bytes(mem.get(addr + offset * 2)) as u64,
                4 => u32::from_le_bytes(mem.get(addr + offset * 4)) as u64,
                8 => u64::from_le_bytes(mem.get(addr + offset * 8)),
                _ => unimplemented!(),
            };
    
            store[segment][offset] = element;
        }
    }


    for nth_reg in 0..nf {
        let vreg = v
            .get(vd + nth_reg, vec_engine)
            .iter_eew()
            .enumerate()
            .masked_map(v.default_mask(vm, vec_engine), v.get(vd, vec_engine).iter_eew(), |(index, _)| {
                store[nth_reg][index]
            })
            .collect_with_eew(vec_engine.sew.clone());

        v.apply(vd, vreg, vec_engine);
    }
    
    todo!("not finished yet");
}
