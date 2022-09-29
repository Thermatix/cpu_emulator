#[derive(Debug, Clone, Copy)]
pub struct OpCode(u8, u8);

pub const NIBBLE: u8 = 4;
pub const BYTE: u8 = 8;
pub const OPCODELENGTH: usize = 2;

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

    // Halt execution
    pub fn halt() -> Self {
        Self (0x00, 0x00)
    }

    // Calls machine code routine (RCA 1802 for COSMAC VIP) at address NNN. Not necessary for most ROMs
    pub fn call_r(n1: u8, n2: u8, n3: u8) -> Self {
        Self (0x0 << NIBBLE | n1, n2 << NIBBLE | n3)
    }

    // Clears the screen
    pub fn clear() -> Self {
        Self (0x00, 0xE0)
    }

    // Returns from a subroutine.
    pub fn ret() -> Self {
        Self (0x00, 0xEE)
    }

    // Jumps to address NNN
    pub fn goto(n1: u8, n2: u8, n3: u8) -> Self {
        Self (0x1 << NIBBLE | n1, n2 << NIBBLE | n3)
    }

    // Calls subroutine at NNN
    pub fn call(n1: u8, n2: u8, n3: u8) -> Self {
        Self (0x2 << NIBBLE | n1, n2 << NIBBLE | n3)
    }

    // Skips the next instruction if VX equals NN
    pub fn skip_x_eq_nn(x: u8, n2: u8, n3: u8) -> Self {
        Self (0x3 << NIBBLE | x, n2 << NIBBLE | n3)
    }

    // 	Skips the next instruction if VX does not equal NN
    pub fn skip_x_neq_nn(x: u8, n2: u8, n3: u8) -> Self {
        Self (0x4 << NIBBLE | x, n2 << NIBBLE | n3)
    }

    // Skips the next instruction if VX equals VY
    pub fn skip_x_eq_y(x: u8, y: u8) -> Self {
        Self (0x5 << NIBBLE | x, y << NIBBLE | 0x0)
    }

    // Sets VX to NN
    pub fn set_x_to_nn(x: u8, n2: u8, n3: u8) -> Self {
        Self (0x6 << NIBBLE | x, n2 << NIBBLE | n3)
    }

    // Adds NN to VX
    pub fn add_nn_to_x(x: u8, n2: u8, n3: u8) -> Self {
        Self (0x7 << NIBBLE | x, n2 << NIBBLE | n3)
    }

    // Sets VX to the value of VY
    pub fn set_x_to_y(x: u8, y: u8) -> Self {
        Self (0x8 << NIBBLE | x, (y << NIBBLE) | 0x0)
    }

    // Sets VX to VX or VY
    pub fn or(x: u8, y: u8) -> Self {
        Self (0x8 << NIBBLE | x, (y << NIBBLE) | 0x1)
    }

    // Sets VX to VX and VY
    pub fn and(x: u8, y: u8) -> Self {
        Self (0x8 << NIBBLE | x, (y << NIBBLE) | 0x2)
    }

    // Sets VX to VX xor VY
    pub fn xor(x: u8, y: u8) -> Self {
        Self (0x8 << NIBBLE | x, (y << NIBBLE) | 0x3)
    }

    // Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not
    pub fn add(x: u8, y: u8) -> Self {
        Self (0x8 << NIBBLE | x, (y << NIBBLE) | 0x4)
    }

    // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not
    pub fn sub(x: u8, y: u8) -> Self {
        Self (0x8 << NIBBLE | x, (y << NIBBLE) | 0x5)
    }

    // Stores the least significant bit of VX in VF and then shifts VX to the right by 1
    pub fn shift_right(x: u8, y: u8) -> Self {
        Self (0x8 << NIBBLE | x, (y << NIBBLE) | 0x6)
    }

    // Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not
    pub fn sub_x(x: u8, y: u8) -> Self {
        Self (0x8 << NIBBLE | x, (y << NIBBLE) | 0x7)
    }

    // Stores the most significant bit of VX in VF and then shifts VX to the left by 1
    pub fn shift_left(x: u8, y: u8) -> Self {
        Self (0x8 << NIBBLE | x, (y << NIBBLE) | 0xE)
    }
    // Skips the next instruction if VX does not equal VY
    pub fn skip_x_neq_y(x: u8, y: u8) -> Self {
        Self (0x9 << NIBBLE | x, y << NIBBLE | 0x0)
    }

    // Sets I to the address NNN
    pub fn set_i_to_nnn(n1: u8, n2: u8, n3: u8) -> Self {
        Self (0xA << NIBBLE | n1, n2 << NIBBLE | n3)
    }

    // Jumps to the address NNN plus V0
    pub fn jump_to_nnn_plus_v0(n1: u8, n2: u8, n3: u8) -> Self {
        Self (0xB << NIBBLE | n1, n2 << NIBBLE | n3)
    }

    // Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN
    pub fn rand(x: u8, n2: u8, n3: u8) -> Self {
        Self (0xC << NIBBLE | x, n2 << NIBBLE | n3)
    }

    // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
    // Each row of 8 pixels is read as bit-coded starting from memory location I;
    // I value does not change after the execution of this instruction. As described above, VF is
    // set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn,
    // and to 0 if that does not happen
    pub fn draw(x: u8, y: u8, n3: u8) -> Self {
        Self (0xD << NIBBLE | x, y << NIBBLE | n3)
    }

    // Skips the next instruction if the key stored in VX is pressed
    pub fn skip_if_key(x: u8) -> Self {
        Self (0xE << NIBBLE | x, 0x9E)
    }

    // Skips the next instruction if the key stored in VX is not pressed
    pub fn skip_if_nkey(x: u8) -> Self {
        Self (0xE << NIBBLE | x, 0xA1)
    }

    // Sets VX to the value of the delay timer
    pub fn set_x_to_timer(x: u8) -> Self {
        Self (0xF << NIBBLE | x, 0x07)
    }

    // A key press is awaited, and then stored in VX. (Blocking Operation)
    pub fn await_key(x: u8) -> Self {
        Self (0xF << NIBBLE | x, 0x0A)
    }

    // Sets the delay timer to VX
    pub fn set_timer_to_x(x: u8) -> Self {
        Self (0xF << NIBBLE | x, 0x15)
    }

    // Sets the sound timer to VX
    pub fn set_sound_timer(x: u8) -> Self {
        Self (0xF << NIBBLE | x, 0x18)
    }

    // Adds VX to I. VF is not affected
    pub fn add_x_to_i(x: u8) -> Self {
        Self (0xF << NIBBLE | x, 0x1E)
    }

    // Sets I to the location of the sprite for the character in VX.
    // Characters 0-F (in hexadecimal) are represented by a 4x5 font.
    pub fn set_i_to_sprite_addr(x: u8) -> Self {
        Self (0xF << NIBBLE | x, 0x29)
    }

    // Stores the binary-coded decimal representation of VX, with the most significant of
    // three digits at the address in I, the middle digit at I plus 1, and the least significant
    // digit at I plus 2. (In other words, take the decimal representation of VX, place the
    // hundreds digit in memory at location in I, the tens digit at location I+1, and the ones
    // digit at location I+2.);
    pub fn store_bcd(x: u8) -> Self {
        Self (0xF << NIBBLE | x, 0x33)
    }

    // Stores from V0 to VX (including VX) in memory, starting at address I.
    // The offset from I is increased by 1 for each value written, but I itself is left unmodified
    pub fn store_0_to_x_to_mem(x: u8) -> Self {
        Self (0xF << NIBBLE | x, 0x55)
    }

    // ills from V0 to VX (including VX) with values from memory, starting at address I.
    // The offset from I is increased by 1 for each value written, but I itself is left unmodified
    pub fn fill_0_to_x_to_mem(x: u8) -> Self {
        Self (0xF << NIBBLE | x, 0x65)
    }
}
