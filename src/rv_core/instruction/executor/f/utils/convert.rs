
// Single precision and rest decomposition
pub fn decompose(f: f64) -> (f32, u32) {
    let bytes = f.to_le_bytes();
    let (low_bytes, high_bytes) = (
        bytes[0 .. 3].try_into().unwrap(), 
        bytes[4 .. 7].try_into().unwrap()
    );
    
    (
        f32::from_le_bytes(low_bytes),
        u32::from_le_bytes(high_bytes)
    )
}

// Double precision recombination
pub fn compose(float: f32, int: u32) -> f64 {
    let low_bytes = float.to_le_bytes();
    let high_bytes = int.to_le_bytes();

    let mut bytes = [0; 8];

    bytes[0 .. 7].copy_from_slice([&low_bytes[..], &high_bytes[..]].concat().as_slice());

    f64::from_le_bytes(bytes)
}