const BASE: u32 = 0b0_01111110_00000000000000000000000;

fn fake(seed: u8) -> f32 {
    let large_number = (seed as u32) << 15;
    let m = f32::from_bits(BASE | large_number);
    2.0 * (m - 0.5)
}
