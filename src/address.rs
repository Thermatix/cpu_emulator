use std::ops::{Add, AddAssign, Sub, SubAssign};
use super::opcodes::{BYTE, NIBBLE};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Address (pub u8, pub u8, pub u8);

impl From<(&u8, &u8, &u8)> for Address {
    fn from(nibbles: (&u8, &u8, &u8)) -> Self {
        Self (*nibbles.0, *nibbles.1, *nibbles.2)
    }
}

impl Add<u16> for Address {
    type Output = Self;

    fn add(self, other: u16) -> Self {
        let s: u16 = self.into();
        (s + other).into()
    }
}

impl AddAssign<u16> for Address {
    fn add_assign(&mut self, other: u16) {
        *self = *self + other;
    }
}

impl Sub<u16> for Address {
    type Output = Self;

    fn sub(self, other: u16) -> Self {
        let s: u16 = self.into();
        (s - other).into()
    }
}

impl SubAssign<u16> for Address {
    fn sub_assign(&mut self, other: u16) {
        *self = *self - other;
    }
}

impl From<u16> for Address {
    fn from(n: u16) -> Self {
        Self (((n & 0x0F00) >> BYTE) as u8, ((n & 0x00F0) >> NIBBLE) as u8, (n & 0x000F >> 0) as u8)
    }
}

impl From<Address> for u16 {
    fn from(constant: Address) -> Self {
        (constant.0 as u16) << BYTE | (constant.1 as u16) << NIBBLE | (constant.1 as u16)  
    }
}

impl From<Address> for usize {
    fn from(constant: Address) -> Self {
        (constant.0 as usize) << BYTE | (constant.1 as usize) << NIBBLE | (constant.1 as usize)  
    }
}

impl From<&Address> for usize {
    fn from(constant: &Address) -> Self {
        (constant.0 as usize) << BYTE | (constant.1 as usize) << NIBBLE | (constant.1 as usize)  
    }
}
impl From<usize> for Address {
    fn from(n: usize) -> Self {
        Self (((n & 0x0F00) >> BYTE) as u8, ((n & 0x00F0) >> NIBBLE) as u8, (n & 0x000F >> 0) as u8)
    }
}

impl Add<usize> for Address {
    type Output = Self;

    fn add(self, other: usize) -> Self {
        let s: usize = self.into();
        (s + other).into()
    }
}

impl AddAssign<usize> for Address {
    fn add_assign(&mut self, other: usize) {
        *self = *self + other;
    }
}

impl Sub<usize> for Address {
    type Output = Self;

    fn sub(self, other: usize) -> Self {
        let s: usize = self.into();
        (s - other).into()
    }
}

impl SubAssign<usize> for Address {
    fn sub_assign(&mut self, other: usize) {
        *self = *self - other;
    }
}
impl PartialEq<usize> for Address {
    fn eq(&self, other: &usize) -> bool {
        let s: usize = self.into();
        s == *other
    }
}
