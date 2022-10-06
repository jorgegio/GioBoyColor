use super::memory_map::*;

const VRAM_SIZE: usize = (VRAM_END - VRAM_START + 1) as usize;

pub struct Gpu {
    vram: [u8; VRAM_SIZE],
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            vram: [0; VRAM_SIZE],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        // TODO: Check for mode, and handle all that (maybe add an unsafe_read? not sure)
        self.vram[address as usize]
    }
}