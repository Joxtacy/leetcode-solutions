use std::ops::{Add, Sub};

#[derive(PartialEq, PartialOrd, Clone, Copy)]
#[repr(u16)]
pub enum Roman {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

impl TryFrom<char> for Roman {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'I' | 'i' => Ok(Roman::I),
            'V' | 'v' => Ok(Roman::V),
            'X' | 'x' => Ok(Roman::X),
            'L' | 'l' => Ok(Roman::L),
            'C' | 'c' => Ok(Roman::C),
            'D' | 'd' => Ok(Roman::D),
            'M' | 'm' => Ok(Roman::M),
            _ => Err(format!("'{c}' is not a valid roman numeral")),
        }
    }
}

// Roman + i32
impl Add<i32> for Roman {
    type Output = i32;

    fn add(self, other: i32) -> i32 {
        self as i32 + other
    }
}

// i32 + Roman
impl Add<Roman> for i32 {
    type Output = i32;

    fn add(self, other: Roman) -> i32 {
        self + other as i32
    }
}

// Roman - i32
impl Sub<i32> for Roman {
    type Output = i32;

    fn sub(self, other: i32) -> i32 {
        self as i32 - other
    }
}

// i32 - Roman
impl Sub<Roman> for i32 {
    type Output = i32;

    fn sub(self, other: Roman) -> i32 {
        self - other as i32
    }
}

impl From<Roman> for i32 {
    fn from(r: Roman) -> i32 {
        r as i32
    }
}
