use std::path::PathBuf;

use crate::core::{gpu::Gpu, memory_map::*, registers::Registers, rom::Rom};

const WRAM_SIZE: usize = (WRAM_END - WRAM_START + 1) as usize;
const HRAM_SIZE: usize = (HRAM_END - HRAM_START + 1) as usize;

pub struct GioBoyColor {
    pub rom: Rom,
    pub ram: [u8; WRAM_SIZE],
    pub hram: [u8; HRAM_SIZE],
    pub gpu: Gpu,
    pub registers: Registers,
}

impl GioBoyColor {
    pub fn new() -> GioBoyColor {
        GioBoyColor {
            rom: Rom::new(),
            ram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            gpu: Gpu::new(),
            registers: Registers::new(),
        }
    }
    pub fn load_rom(&mut self, rom_path: &PathBuf) {
        self.rom.load(rom_path);
    }
    fn read(&self, address: u16) -> u8 {
        // TODO: Check if a register has been requested

        match address {
            ROM_START..=ROM_END => self.rom.read(address),
            VRAM_START..=VRAM_END => self.gpu.read(address),
            ERAM_START..=ERAM_END => self.rom.read(address),
            WRAM_START..=WRAM_END => self.ram[(address - WRAM_START) as usize],
            ECHO_START..=ECHO_END => self.ram[(address - ECHO_START) as usize],
            OAM_START..=OAM_END => self.gpu.read(ECHO_START),
            HRAM_START..=HRAM_END => self.hram[(address - WRAM_START) as usize],
            _ => panic!("Attempted to read from an invalid address: {}", address),
        }
    }

    fn write(&mut self, address: u16, data: u8) {
        // TODO: Check write to registers

        match address {
            ROM_START..=ROM_END => self.rom.write(address, data),
            VRAM_START..=VRAM_END => self.gpu.write(address, data),
            ERAM_START..=ERAM_END => self.rom.write(address, data),
            WRAM_START..=WRAM_END => self.ram[(address - WRAM_START) as usize] = data,
            ECHO_START..=ECHO_END => {
                // Note: Use of the area from 0xE000 to 0xFDFF is prohibited.
                self.ram[(address - ECHO_START) as usize] = data;
                panic!("Attempted to write to ECHO RAM");
            }
            OAM_START..=OAM_END => self.gpu.write(address, data),
            HRAM_START..=HRAM_END => self.hram[(address - HRAM_START) as usize] = data,
            _ => panic!("Attempted to write at an invalid address: {}", address),
        }
    }

    fn next_byte(&mut self) -> u8 {
        let byte = self.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        byte
    }

    // Returns the next word (pointer, little endian)
    fn next_pointer(&mut self) -> u16 {
        let low = self.next_byte();
        let high = self.next_byte();
        combine!(high, low)
    }

    // Perform a CPU step, returns machine cycles
    pub fn step(&mut self) -> usize {
        // Read opcode at PC
        let opcode = self.next_byte();

        // Execute instruction
        self.execute_instruction(opcode)
    }

    // Pushes 16 bit data onto the stack
	fn push(&mut self, data: u16) {
		self.registers.sp = self.registers.sp.wrapping_sub(1);
		self.write(self.registers.sp, high!(data));
		self.registers.sp = self.registers.sp.wrapping_sub(1);
		self.write(self.registers.sp, low!(data));
	}

	// Pops highest 16 bits from stack
	fn pop(&mut self) -> u16 {
		let low = self.read(self.registers.sp);
		self.registers.sp = self.registers.sp.wrapping_add(1);
		let high = self.read(self.registers.sp);
		self.registers.sp = self.registers.sp.wrapping_add(1);
		combine!(high, low)
	}

    #[rustfmt::skip]
    fn execute_instruction(&mut self, opcode: u8) -> usize {
    match opcode {
        0x00 => { 1 },   // NOP
        0x10 => { 1 },  // STOP

        //  8-bit load instructions
        // LD r, r'
        0x78 => { self.registers.a = self.registers.b; 1 }, // LD A, B
        0x79 => { self.registers.a = self.registers.c; 1 }, // LD A, C
        0x7a => { self.registers.a = self.registers.d; 1 }, // LD A, D
        0x7b => { self.registers.a = self.registers.e; 1 }, // LD A, E
        0x7c => { self.registers.a = self.registers.h; 1 }, // LD A, H
        0x7d => { self.registers.a = self.registers.l; 1 }, // LD A, L
        0x7f => { 1 },                                    // LD A, A
        0x47 => { self.registers.b = self.registers.a; 1 }, // LD B, A
        0x40 => { 1 },                                    // LD B, B
        0x41 => { self.registers.b = self.registers.c; 1 }, // LD B, C
        0x42 => { self.registers.b = self.registers.d; 1 }, // LD B, D
        0x43 => { self.registers.b = self.registers.e; 1 }, // LD B, E
        0x44 => { self.registers.b = self.registers.h; 1 }, // LD B, H
        0x45 => { self.registers.b = self.registers.l; 1 }, // LD B, L
        0x4f => { self.registers.c = self.registers.a; 1 }, // LD C, A
        0x48 => { self.registers.c = self.registers.b; 1 }, // LD C, B
        0x49 => { 1 },                                    // LD C, C
        0x4a => { self.registers.c = self.registers.d; 1 }, // LD C, D
        0x4b => { self.registers.c = self.registers.e; 1 }, // LD C, E
        0x4c => { self.registers.c = self.registers.h; 1 }, // LD C, H
        0x4d => { self.registers.c = self.registers.l; 1 }, // LD C, L
        0x57 => { self.registers.d = self.registers.a; 1 }, // LD D, A
        0x50 => { self.registers.d = self.registers.b; 1 }, // LD D, B
        0x51 => { self.registers.d = self.registers.c; 1 }, // LD D, C
        0x52 => { 1 },                                    // LD D, D
        0x53 => { self.registers.d = self.registers.e; 1 }, // LD D, E
        0x54 => { self.registers.d = self.registers.h; 1 }, // LD D, H
        0x55 => { self.registers.d = self.registers.l; 1 }, // LD D, L
        0x5F => { self.registers.e = self.registers.a; 1 }, // LD E, A
        0x58 => { self.registers.e = self.registers.b; 1 }, // LD E, B
        0x59 => { self.registers.e = self.registers.c; 1 }, // LD E, C
        0x5a => { self.registers.e = self.registers.d; 1 }, // LD E, D
        0x5b => { 1 },                                    // LD E, E
        0x5c => { self.registers.e = self.registers.h; 1 }, // LD E, H
        0x5d => { self.registers.e = self.registers.l; 1 }, // LD E, L
        0x67 => { self.registers.h = self.registers.a; 1 }, // LD H, A
        0x60 => { self.registers.h = self.registers.b; 1 }, // LD H, B
        0x61 => { self.registers.h = self.registers.c; 1 }, // LD H, C
        0x62 => { self.registers.h = self.registers.d; 1 }, // LD H, D
        0x63 => { self.registers.h = self.registers.e; 1 }, // LD H, E
        0x64 => { 1 },                                    // LD H, H
        0x65 => { self.registers.h = self.registers.l; 1 }, // LD H, E
        0x6f => { self.registers.l = self.registers.a; 1 }, // LD L, A
        0x68 => { self.registers.l = self.registers.b; 1 }, // LD L, B
        0x69 => { self.registers.l = self.registers.c; 1 }, // LD L, C
        0x6a => { self.registers.l = self.registers.d; 1 }, // LD L, D
        0x6b => { self.registers.l = self.registers.e; 1 }, // LD L, E
        0x6c => { self.registers.l = self.registers.h; 1 }, // LD L, H
        0x6d => { 1 },                                    // LD L, L
        // LD r, n
        0x3e => { self.registers.a = self.next_byte(); 2 }, // LD A, n
        0x06 => { self.registers.b = self.next_byte(); 2 }, // LD B, n
        0x0e => { self.registers.c = self.next_byte(); 2 }, // LD C, n
        0x16 => { self.registers.d = self.next_byte(); 2 }, // LD D, n
        0x1e => { self.registers.e = self.next_byte(); 2 }, // LD E, n
        0x26 => { self.registers.h = self.next_byte(); 2 }, // LD H, n
        0x2e => { self.registers.l = self.next_byte(); 2 }, // LD L, n
        // LD r, (HL)
        0x7e => { self.registers.a = self.read(self.registers.hl()); 2 },
        0x46 => { self.registers.b = self.read(self.registers.hl()); 2 },
        0x4e => { self.registers.c = self.read(self.registers.hl()); 2 },
        0x56 => { self.registers.d = self.read(self.registers.hl()); 2 },
        0x5e => { self.registers.e = self.read(self.registers.hl()); 2 },
        0x66 => { self.registers.h = self.read(self.registers.hl()); 2 },
        0x6e => { self.registers.l = self.read(self.registers.hl()); 2 },
        // LD (HL), r
        0x77 => { self.write(self.registers.hl(), self.registers.a); 2 },
        0x70 => { self.write(self.registers.hl(), self.registers.b); 2 },
        0x71 => { self.write(self.registers.hl(), self.registers.c); 2 },
        0x72 => { self.write(self.registers.hl(), self.registers.d); 2 },
        0x73 => { self.write(self.registers.hl(), self.registers.e); 2 },
        0x74 => { self.write(self.registers.hl(), self.registers.h); 2 },
        0x75 => { self.write(self.registers.hl(), self.registers.l); 2 },
        // LD (HL), n
        0x36 => { let n = self.next_byte(); self.write(self.registers.hl(), n); 3 },
        // LD A, (BC)
        0x0a => { self.registers.a = self.read(self.registers.bc()); 2 },
        // LD A, (DE)
        0x1a => { self.registers.a = self.read(self.registers.de()); 2 },
        // LD (BC), A 
        0x02 => { self.write(self.registers.bc(), self.registers.a); 2 },
        // LD (DE), A 
        0x12 => { self.write(self.registers.de(), self.registers.a); 2 },
        // LD A, (nn) 
        0xfa => { let nn = self.next_pointer(); self.registers.a = self.read(nn); 4 },
        // LD (nn), A 
        0xea => { let nn = self.next_pointer(); self.write(nn, self.registers.a); 4 },
        // LDH A, (C)
        0xf2 => { self.registers.a = self.read(combine!(0xff, self.registers.c)); 2 },
        // LDH (C), A
        0xe2 => { self.write(combine!(0xff, self.registers.c), self.registers.a); 2 },
        // LDH A, n
        0xf0 => { let n = self.next_byte(); self.registers.a = self.read(combine!(0xff, n)); 3 },
        // LDH (n), A
        0xe0 => { let n = self.next_byte(); self.write(combine!(0xff, n), self.registers.a); 3 },
        // LD A, (HL-)
        0x3a => { self.registers.a = self.read(self.registers.hl()); self.registers.hld(); 2 },
        // LDH (HL-), A
        0x32 => { self.write(self.registers.hl(), self.registers.a); self.registers.hld(); 2 },
        // LD A, (HL+)
        0x2a => { self.registers.a = self.read(self.registers.hl()); self.registers.hli(); 2 },
        // LDH (HL+), A
        0x22 => { self.write(self.registers.hl(), self.registers.a); self.registers.hli(); 2 },

        //  16-bit load instructions
        // LD rr, nn
        0x01 => { let nn = self.next_pointer(); self.registers.set_bc(nn); 3 } // LD BC, nn
        0x11 => { let nn = self.next_pointer(); self.registers.set_de(nn); 3 } // LD DE, nn
        0x21 => { let nn = self.next_pointer(); self.registers.set_hl(nn); 3 } // LD HL, nn
        0x31 => { let nn = self.next_pointer(); self.registers.sp = nn; 3 } // LD SP, nn
        // LD (nn), SP
        0x08 => { 
            let nn = self.next_pointer(); 
            self.write(nn, low!(self.registers.sp)); 
            self.write(nn + 1, high!(self.registers.sp));  
            5 
        },
        // LD SP, HL
        0xf9 => { self.registers.sp = self.registers.hl() ; 2 } 
        // PUSH rr
        0xc5 => { self.push(self.registers.bc()); 4 } // PUSH BC
        0xd5 => { self.push(self.registers.de()); 4 } // PUSH DE
        0xe5 => { self.push(self.registers.hl()); 4 } // PUSH HL
        0xf5 => { self.push(self.registers.af()); 4 } // PUSH AF
        // POP rr
        0xc1 => { let val = self.pop(); self.registers.set_bc(val); 3 },
        0xd1 => { let val = self.pop(); self.registers.set_de(val); 3 },
        0xe1 => { let val = self.pop(); self.registers.set_hl(val); 3 },
        0xf1 => {
            // Lower 4 bits of register F (unused flag bits) are set to zero
            let val = self.pop() & 0xFFF0;
            self.registers.set_af(val);
            3
        },
        _ => { panic!("Unsupported operation {:02X}", opcode); }
    }
}
}
