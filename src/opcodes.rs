pub struct OpCode(u8, u8);

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
        (c.0 as u16) << 8 | c.1 as u16
    }
}

impl From<&OpCode> for u16 {
    fn from(c: &OpCode) -> u16 {
        c.into()
    }
}

impl OpCode {
    pub fn add(h: u8, l: u8) -> Self {
        Self (0x8 << 4 | h, (l << 4) | 4)
    }

    pub fn r#return() -> Self {
        Self (0x00, 0x00)
    }
}
