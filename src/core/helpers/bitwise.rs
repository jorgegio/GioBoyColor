pub enum Bit {
    Bit0 = 0b00000001,
    Bit1 = 0b00000010,
    Bit2 = 0b00000100,
    Bit3 = 0b00001000,
    Bit4 = 0b00010000,
    Bit5 = 0b00100000,
    Bit6 = 0b01000000,
    Bit7 = 0b10000000,
}

macro_rules! clear {
    ($byte:expr, u8) => {
        $byte = 0x00;
    };
}

macro_rules! set_bit {
    (($byte:expr, u8), ($b:expr, Bit)) => {
        byte = byte | b as u8;
    };
}

macro_rules! clear_bit {
    (($byte:expr, u8), ($b:expr, Bit)) => {
        byte = byte & !(b as u8);
    };
}

macro_rules! is_set {
    (($byte:expr, u8), ($b:expr, Bit)) => {
        (byte & b as u8 > 0)
    };
}

// combine two u8s into a u16
macro_rules! combine {
    ($h:expr, $l:expr) => {
        ((($h as u16) << 8) | $l as u16)
    };
}

// return high byte from u16
macro_rules! high {
    ($word:expr) => {
        (($word >> 8) as u8)
    };
}

// return low byte from u16
macro_rules! low {
    ($word:expr) => {
        (($word & 0xFF) as u8)
    };
}
