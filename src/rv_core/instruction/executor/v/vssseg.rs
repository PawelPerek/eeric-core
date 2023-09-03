use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vss { vs3, rs1, rs2, vm }: Vss,
    eew: SEW,
    nf: usize,
    v: &VectorRegisters,
    vec_engine: &VectorEngine,
    x: &IntegerRegisters,
    mem: &mut Memory,
) {
    let addr = x[rs1] as usize;
    let stride = x[rs2] as usize;

    for segment in 0..nf {
        izip!(v.get(vs3 + segment, vec_engine).iter_custom_eew(eew), v.default_mask(vm, vec_engine))
            .enumerate()
            .for_each(|(index, (value, mask))| {
                let segment_size = nf.wrapping_mul(eew.byte_length());
                let segment_index = segment.wrapping_mul(eew.byte_length());

                let offset = index.wrapping_mul(segment_size).wrapping_add(segment_index).wrapping_mul(stride);

                let address = addr.wrapping_add(offset);
                if mask == 1 {
                    match eew {
                        SEW::E8 => mem.set(address, (value as u8).to_le_bytes()),
                        SEW::E16 => mem.set(address, (value as u16).to_le_bytes()),
                        SEW::E32 => mem.set(address, (value as u32).to_le_bytes()),
                        SEW::E64 => mem.set(address, (value as u64).to_le_bytes()),
                        _ => unimplemented!()
                    };
                }
            });
    }
}
