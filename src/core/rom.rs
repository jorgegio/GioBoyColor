use std::{vec::Vec, fs::File, io::Read, path::PathBuf, fmt};

pub struct Rom {
    bytes: Vec<u8>,
    pub is_loaded: bool
}

impl Rom {
    pub fn new() -> Rom {
        Rom {
            bytes: Vec::new(),
            is_loaded: false
        }
    }

    pub fn load(&mut self, rom_path: &PathBuf) {
        let mut file = File::open(rom_path).expect("Invalid ROM path");
		file.read_to_end(&mut self.bytes).expect("Unable to read ROM");
        println!("Successfully loaded ROM:\n{}", self);
        self.is_loaded = true;
    }

    pub fn read(&self, address: u16) -> u8 {
        self.bytes[address as usize]
    }

    pub fn write(&self, address: u16, data: u8) {
        todo!()
    }

    fn title(&self) -> String {
        let mut name = String::new();
        
        for index in 0x134..0x144 {
            let code = self.read(index);

            match code {
                0 => break,
                _ => name.push(code as char)
            }
        }
        
        return name;
    }

    fn cgb_flag(&self) -> u8 {
        self.read(0x143)
    }

    fn sgb_flag(&self) -> u8 {
        self.read(0x146)
    }

    fn cartridge_type(&self) -> u8 {
        self.read(0x147)
    }
    
    fn rom_size(&self) -> u8 {
        self.read(0x148)
    }

    fn ram_size(&self) -> u8 {
        self.read(0x149)
    }
}

impl fmt::Display for Rom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, 
"*~~ ROM Header ~~*
Title: {}
CGB Flag: {}
SGB Flag: {}
Cartridge Type: {}
ROM Size: {}
RAM Size: {}
",
            self.title(),
            match self.cgb_flag() {
                0x80 => "The game supports CGB enhancements, but is backwards compatible with monochrome Game Boys",
                0xC0 => "The game works on CGB only (the hardware ignores bit 6, so this really functions the same as $80)",
                _ => "UNKNOWN CGB FLAG"
            },
            match self.sgb_flag() {
				0x03 => "The game supports SGB functions",
                _ => "The game doesn't support SGB functions"
            },
            match self.cartridge_type() {
                0x00 =>	"ROM ONLY",
                0x01 =>	"MBC1",
                0x02 =>	"MBC1+RAM",
                0x03 =>	"MBC1+RAM+BATTERY",
                0x05 =>	"MBC2",
                0x06 =>	"MBC2+BATTERY",
                0x08 =>	"ROM+RAM 1",
                0x09 =>	"ROM+RAM+BATTERY 1",
                0x0B =>	"MMM01",
                0x0C =>	"MMM01+RAM",
                0x0D =>	"MMM01+RAM+BATTERY",
                0x0F =>	"MBC3+TIMER+BATTERY",
                0x10 =>	"MBC3+TIMER+RAM+BATTERY 2",
                0x11 =>	"MBC3",
                0x12 =>	"MBC3+RAM 2",
                0x13 =>	"MBC3+RAM+BATTERY 2",
                0x19 =>	"MBC5",
                0x1A =>	"MBC5+RAM",
                0x1B =>	"MBC5+RAM+BATTERY",
                0x1C =>	"MBC5+RUMBLE",
                0x1D =>	"MBC5+RUMBLE+RAM",
                0x1E =>	"MBC5+RUMBLE+RAM+BATTERY",
                0x20 =>	"MBC6",
                0x22 =>	"MBC7+SENSOR+RUMBLE+RAM+BATTERY",
                0xFC =>	"POCKET CAMERA",
                0xFD =>	"BANDAI TAMA5",
                0xFE =>	"HuC3",
                0xFF =>	"HuC1+RAM+BATTERY",
                _ =>    "UNKNOWN CARTRIDGE TYPE"
            },
            match self.rom_size() {
                0x00 =>	"32 KiB	(no banking)",
                0x01 =>	"64 KiB	(4 banks)",
                0x02 =>	"128 KiB (8 banks)",
                0x03 =>	"256 KiB (16 banks)",
                0x04 =>	"512 KiB (32 banks)",
                0x05 =>	"1 MiB (64 banks)",
                0x06 =>	"2 MiB (128 banks)",
                0x07 =>	"4 MiB (256 banks)",
                0x08 =>	"8 MiB (512 banks)",
                0x52 =>	"1.1 MiB (72 banks)",
                0x53 =>	"1.2 MiB (80 banks)",
                0x54 =>	"1.5 MiB (96 banks)",
                _ => "UNKNOWN ROM_SIZE"
            },
            match self.ram_size() {
                0x00 =>	"0 (No RAM)",
                0x01 =>	"Unused",
                0x02 =>	"8 KiB (1 bank)",
                0x03 =>	"32 KiB (4 banks of 8 KiB each)",
                0x04 =>	"128 KiB (16 banks of 8 KiB each)",
                0x05 =>	"64 KiB	(8 banks of 8 KiB each)",
                _ => "UNKNOWN RAM SIZE"
            }
        )
    }
}