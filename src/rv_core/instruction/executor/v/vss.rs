use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vss { vs3, rs1, rs2, vm }: Vss,
    eew: SEW,
    v: &VectorRegisters,
    vec_engine: &VectorEngine,
    x: &IntegerRegisters,
    mem: &mut Memory,
) {
    let addr = x[rs1] as usize;
    let stride = x[rs2] as usize;

    izip!(
        v.get(vs3, vec_engine).iter_custom_eew(eew.clone()),
        v.default_mask(vm, &vec_engine)
    )
    .enumerate()
    .for_each(|(index, (vs3, mask))| {
        let address = addr.wrapping_add(index.wrapping_mul(stride).wrapping_mul(eew.byte_length()));

        if mask == 1 {
            match eew {
                SEW::E8 => mem.set(address, (vs3 as u8).to_le_bytes()),
                SEW::E16 => mem.set(address, (vs3 as u16).to_le_bytes()),
                SEW::E32 => mem.set(address, (vs3 as u32).to_le_bytes()),
                SEW::E64 => mem.set(address, (vs3 as u64).to_le_bytes()),
                SEW::E128 => unimplemented!(),
            }
        }
    });
}
