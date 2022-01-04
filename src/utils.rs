pub fn get_nth_nibble(val: u16, n: u8) -> u16 {
    // n=0 is first nibble
    (val & (0xF << n*4)) >> n*4
}

pub fn get_first_n_nibbles(val: u16, n: u8) -> u16 {
    let mut new_val: u16 = 0;

    for i in 0..n {
        new_val |= val & (0xF << i*4);
    }

    new_val
}

pub fn concat_bytes(b1: u8, b2: u8) -> u16 {
    ((b1 as u16) << 8) | b2 as u16
}

pub struct ArithmeticOverflow{
    pub val: u8,
    pub overflowed: bool,
}

pub fn addition_with_overflow(a: u8, b: u8) -> ArithmeticOverflow {
    let mut sum = a as u16 + b as u16;
    let overflowed = sum > 255;
    sum -= (sum / 256) * 256;

    ArithmeticOverflow{val: sum as u8, overflowed}
}

pub fn subtract_with_overflow(a: u8, b: u8) -> ArithmeticOverflow {
    let mut sub: i16 = a as i16 - b as i16;
    let overflowed = sub < 0;

    if overflowed {sub += 256};

    ArithmeticOverflow{val: sub as u8, overflowed}
}

pub fn to_binary_encoded_decimal(val: u8, len: usize) -> Vec<u8> {
    let mut vec = Vec::new();

    // let mut ret_arr: [u8; 3] = [0; 3];

    for int_char in val.to_string().chars() {
        vec.push(int_char.to_digit(10).unwrap() as u8);
    }

    // pad front with zeros
    let pad_len = len - vec.len();
    for _ in 0..pad_len {
        vec.push(0);
    }

    vec
}