const BIAS: i16 = 128;
const RADIX: f32 = 2.0;

pub struct PARTSU32(u32, u32, u32);

impl From<f32> for PARTSU32 {
    fn from(n: f32) -> Self {
        let bits = n.to_bits();

        let sign     = (bits >> 31) & 1;
        let exponent = (bits >> 23) & 0xff;
        let fraction = bits & 0x7fffff;

        PARTSU32(sign, exponent, fraction)
    }
}


impl From<PARTSU32> for f32 {
    fn from(p: PARTSU32) -> Self {

        let sign = (-1.0_f32).powf(p.0 as f32);

        let exponent = (p.1 as i32) - BIAS as i32;
        let exponent = RADIX.powf(exponent as f32);
        let mut mantissa: f32  = 1.0;

        for i in 0..23 {
            let mask = 1 << i;
            let one_at_bit_i = p.2 & mask;
            if one_at_bit_i != 0 {
                let i_ = i as f32;
                let weight = 2_f32.powf( i_ - 23.0 );
                mantissa += weight;
            }
        }
        sign * exponent * mantissa
    }
}

// pub fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
//     let signed_1 = (-1.0_f32).powf(sign as f32);

//     let exponent = (exponent as i32) - BIAS as i32;
//     let exponent = RADIX.powf(exponent as f32);
//     let mut mantissa: f32  = 1.0;

//     for i in 0..23 {
//         let mask = 1 << i;
//         let one_at_bit_i = fraction & mask;
//         if one_at_bit_i != 0 {
//             let i_ = i as f32;
//             let weight = 2_f32.powf( i_ - 23.0 );
//             mantissa += weight;
//         }
//     }

//     (signed_1, exponent, mantissa)
// }

// pub fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
//     sign * exponent * mantissa
//}
