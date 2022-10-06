use std::path::PathBuf;

use super::gpu::Gpu;
use super::rom::Rom;
use super::registers::Registers;

use super::memory_map::*;

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
    pub fn read(&self, address: u16) -> u8 {
        // TODO: Check if a register have been requested

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
}
