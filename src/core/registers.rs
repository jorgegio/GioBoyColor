pub struct Registers {
    pub af: u16,
    pub bc: u16,
	pub de: u16,
	pub hl: u16,
	pub sp: u16,
	pub pc: u16
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
}
