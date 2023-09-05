use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vsx { vs3, rs1, vs2, vm }: Vsx,
    eew: SEW,
    v: &mut VectorContext<'_>,
    x: &IntegerRegisters,
    mem: &mut Memory,
) {
    let base_addr = x[rs1] as usize;

    izip!(
        v.get(vs3).iter_eew(),
        v.get(vs2).iter_custom_eew(eew.clone()),
        v.default_mask(vm)
    )
    .for_each(|(data, offset, mask)| {
        let address = base_addr.wrapping_add(offset as usize);
        if mask == 1 {
            match v.vec_engine.sew {
                SEW::E8 => mem.set(address, (data as u8).to_le_bytes()),
                SEW::E16 => mem.set(address, (data as u16).to_le_bytes()),
                SEW::E32 => mem.set(address, (data as u32).to_le_bytes()),
                SEW::E64 => mem.set(address, (data as u64).to_le_bytes()),
                SEW::E128 => unimplemented!(),
            }
        }
    });
}
