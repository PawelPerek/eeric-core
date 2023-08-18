pub trait NumSetLsbExt {
    fn set_lsb(&mut self, value: u8);
}

impl NumSetLsbExt for u64 {
    fn set_lsb(&mut self, bit: u8) {
        debug_assert!(bit == 0 || bit == 1, "bit must be 0 or 1");

        if bit == 1 {
            *self |= 1;
        } else {
            *self &= !1;
        }
    }
}