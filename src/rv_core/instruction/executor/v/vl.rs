use std::convert::identity;

use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vl { vd, rs1, vm }: Vl,
    eew: SEW,
    x: &IntegerRegisters,
    v: &mut VectorContext<'_>,
    mem: &Memory,
) -> Result<(), String> {
    let addr = x[rs1] as usize;

    let emul = (eew.byte_length() as f32 / v.vec_engine.sew.byte_length() as f32) * v.vec_engine.lmul.ratio(); 

    if !(0.125..=8.0).contains(&emul) {
        return Err(format!("Vector load error: Expected EMUL to be in range <1/8, 8>, got {} instead", emul));
    } 

    let element_amount = v.vlmax_custom_emul(LMUL::try_from(emul)?);

    let vreg = (0..element_amount)
        .map(|offset| addr.wrapping_add(offset.wrapping_mul(eew.byte_length())))
        .map(|address| match eew {
            SEW::E8 => u8::from_le_bytes(mem.get(address)) as u64,
            SEW::E16 => u16::from_le_bytes(mem.get(address)) as u64,
            SEW::E32 => u32::from_le_bytes(mem.get(address)) as u64,
            SEW::E64 => u64::from_le_bytes(mem.get(address)),
            _ => unimplemented!(),
        })
        .masked_map(
            v.default_mask(vm), 
            v.get(vd).iter_custom_eew(eew), 
            identity)
        .collect_with_eew(eew);

    v.apply(vd, vreg);
    
    Ok(())
}
