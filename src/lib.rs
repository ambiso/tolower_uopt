fn tolower_schoen(c: u8) -> u8 {
    c + ((c >= 'A' as u8) & (c <= 'Z' as u8)) as u8 * ('a' as u8 - 'A' as u8)
}

fn tolower_switch(c: u8) -> u8 {
    const A: u8 = 'A' as u8;
    const Z: u8 = 'Z' as u8;
    match c {
        A..=Z => c - ('a' as u8 - 'A' as u8),
        _ => c,
    }
}

const TO_LOWER_LUT: [u8; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 97, 98, 99, 100, 101, 102, 103,
    104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
    91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130,
    131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149,
    150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168,
    169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187,
    188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206,
    207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225,
    226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244,
    245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
];

fn tolower_lut(c: u8) -> u8 {
    TO_LOWER_LUT[c as usize]
}

pub fn tolower_schoen_array(s: &mut [u8]) {
    for c in s {
        *c = tolower_schoen(*c);
    }
}

pub fn tolower_switch_array(s: &mut [u8]) {
    for c in s {
        *c = tolower_switch(*c);
    }
}

pub fn tolower_lut_array(s: &mut [u8]) {
    for c in s {
        *c = tolower_lut(*c);
    }
}

#[cfg(test)]
mod test {
    use crate::{tolower_lut, tolower_schoen, tolower_switch};

    fn test_fn(f: fn(u8) -> u8) {
        for c in 0..256 {
            let expected = (c as u8 as char).to_ascii_lowercase() as u8;
            let got = f(c as u8);
            assert_eq!(
                got, expected,
                "got wrong result for {} ('{}') expected {} ('{}') but got {} ('{}')",
                c, c as u8 as char, expected, expected as u8 as char, got, got as u8 as char,
            );
        }
    }

    #[test]
    fn test_schoen() {
        test_fn(tolower_schoen);
    }

    #[test]
    fn test_switch() {
        test_fn(tolower_switch);
    }

    #[test]
    fn test_lut() {
        test_fn(tolower_lut);
    }
}
