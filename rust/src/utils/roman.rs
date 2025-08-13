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

impl TryFrom<i32> for Roman {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::I),
            5 => Ok(Self::V),
            10 => Ok(Self::X),
            50 => Ok(Self::L),
            100 => Ok(Self::C),
            500 => Ok(Self::D),
            1000 => Ok(Self::M),
            _ => Err(format!("'{value}' is not a valid roman value")),
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

impl From<Roman> for char {
    fn from(r: Roman) -> char {
        match r {
            Roman::I => 'I',
            Roman::V => 'V',
            Roman::X => 'X',
            Roman::L => 'L',
            Roman::C => 'C',
            Roman::D => 'D',
            Roman::M => 'M',
        }
    }
}
