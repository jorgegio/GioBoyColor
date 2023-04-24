/// Registers
///
/// 16-bit	Hi	Lo	Name/Function           \
/// AF	    A	-	Accumulator & Flags     \
/// BC	    B	C	BC                      \
/// DE	    D	E	DE                      \
/// HL	    H	L	HL                      \
/// SP	    -	-	Stack Pointer           \
/// PC	    -	-	Program Counter/Pointer \
///
/// The Flags Register (lower 8 bits of AF register)
/// Bit	Name	Explanation                 \
/// 7	z	    Zero flag                   \
/// 6	n	    Subtraction flag (BCD)      \
/// 5	h	    Half Carry flag (BCD)       \
/// 4	c	    Carry flag                  \
/// Contains information about the result of the most recent instruction that has affected flags.
///
/// The Zero Flag (Z)                       \
/// This bit is set if and only if the result of an operation is zero. Used by conditional jumps.
///
/// The Carry Flag (C, or Cy)               \
/// Is set in these cases:
///
/// When the result of an 8-bit addition is higher than $FF.    \   
/// When the result of a 16-bit addition is higher than $FFFF.  \
/// When the result of a subtraction or comparison is lower than zero (like in Z80 and 80x86 CPUs, but unlike in 65XX and ARM CPUs). \
/// When a rotate/shift operation shifts out a “1” bit.         \
/// Used by conditional jumps and instructions such as ADC, SBC, RL, RLA, etc.  \
///
/// The BCD Flags (N, H)                    \
/// These flags are used by the DAA instruction only.   \
/// N indicates whether the previous instruction has been a subtraction, and H indicates carry for the lower 4 bits of the result.  \
/// DAA also uses the C flag, which must indicate carry for the upper 4 bits.  \
/// After adding/subtracting two BCD numbers, DAA is used to convert the result to BCD format.  \
///
/// BCD numbers range from $00 to $99 rather than $00 to $FF. \
/// Because only two flags (C and H) exist to indicate carry-outs of BCD digits, DAA is ineffective for 16-bit operations (which have 4 digits), and use for INC/DEC operations (which do not affect C-flag) has limits.
///
/// (Docs sourced from [Pan Docs](https://gbdev.io/pandocs/))

pub struct Registers {
    // Hi: A, Lo: -, Accumulator & Flags
    pub a: u8,
    pub f: u8,
    // Hi: B, Lo: C
    pub b: u8,
    pub c: u8,
    // Hi: D, Lo: E
    pub d: u8,
    pub e: u8,
    // Hi: H, Lo: L
    pub h: u8,
    pub l: u8,
    // Stack Pointer
    pub sp: u16,
    // Program Counter
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Registers {
        // Registers are set to these specific values after GB BIOS runs
        Registers {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x0100,
        }
    }

    pub fn af(&self) -> u16 {
        combine!(self.a, self.f)
    }

    pub fn bc(&self) -> u16 {
        combine!(self.b, self.c)
    }

    pub fn de(&self) -> u16 {
        combine!(self.d, self.e)
    }

    pub fn hl(&self) -> u16 {
        combine!(self.h, self.l)
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = high!(value);
        self.f = low!(value);
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = high!(value);
        self.c = low!(value);
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = high!(value);
        self.e = low!(value);
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = high!(value);
        self.l = low!(value);
    }

    pub fn hli(&mut self) {
        let new = self.hl().wrapping_add(1);
        self.set_hl(new);
    }

    pub fn hld(&mut self) {
        let new = self.hl().wrapping_sub(1);
        self.set_hl(new);
    }
}
