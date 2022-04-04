#[derive(Debug, Clone, Copy)]
pub struct OpCode(u8, u8);

pub const NIBBLE: u8 = 4;
pub const BYTE: u8 = 8;

impl OpCode {
    pub fn high_byte(&self) -> u8 {
        self.0
    }

    pub fn low_byte(&self) -> u8 {
        self.1
    }
}

impl From<OpCode> for u16 {
    fn from(c: OpCode) -> Self {
        (c.0 as u16) << BYTE | c.1 as u16
    }
}

impl From<&OpCode> for u16 {
    fn from(c: &OpCode) -> u16 {
        c.into()
    }
}

impl OpCode {
    pub fn halt() -> Self {
        Self (0x00, 0x00)
    }
    pub fn add(h: u8, l: u8) -> Self {
        Self (0x8 << NIBBLE | h, (l << NIBBLE) | NIBBLE)
    }

    pub fn call(n1: u8, n2: u8, n3: u8) -> Self {
        Self (0x2 << NIBBLE | n1, n2 << NIBBLE | n3)
    }

    pub fn ret() -> Self {
        Self (0x00, 0xEE)
    }
}
