pub struct Registers {
    pub af: u16, // Hi: A, Lo: -, Accumulator & Flags
    pub bc: u16, // Hi: B, Lo: C
    pub de: u16, // Hi: D, Lo: E
    pub hl: u16, // Hi: H, Lo: L
    pub sp: u16, // Stack Pointer
    pub pc: u16, // Program Counter
}

impl Registers {
    pub fn new() -> Registers {
        // Registers are set to these specific values after GB BIOS runs
        Registers {
            af: 0x01B0,
            bc: 0x0013,
            de: 0x00D8,
            hl: 0x014D,
            sp: 0xFFFE,
            pc: 0x0100,
        }
    }
}
