use std::fs::File;
use std::io;
use std::io::Read;

// const MEMORY_SIZE: usize = 0x4000;
const MEMORY_SIZE: usize = 0x10000;

#[allow(dead_code)]
const DEBUG_MEMORY_SIZE: usize = 0x10000;

pub struct Mmu {
    memory: Vec<u8>,
}

impl Mmu {
    // pub fn new() -> Mmu {
    pub fn new(rom_h: &[u8; 0x800], rom_g: &[u8; 0x800], rom_f: &[u8; 0x800], rom_e: &[u8; 0x800]) -> Mmu {
        let mut mmu = Mmu {
            memory: vec![0; MEMORY_SIZE],
        };

        mmu.memory[0x0000..0x0800].clone_from_slice(rom_h);
        mmu.memory[0x0800..0x1000].clone_from_slice(rom_g);
        mmu.memory[0x1000..0x1800].clone_from_slice(rom_f);
        mmu.memory[0x1800..0x2000].clone_from_slice(rom_e);

        mmu
    }

    #[allow(dead_code)]
    pub fn new_debug(rom_path: &str) -> Self {
        let mut mmu = Mmu {
            memory: vec![0; DEBUG_MEMORY_SIZE],
        };

        let debug_rom_and_size = read_complete_rom(rom_path).unwrap();
        mmu.memory[0x100..(0x100 + debug_rom_and_size.1)].copy_from_slice(&debug_rom_and_size.0);

        // inject "out 0,a" at 0x0000 (signal to stop the test)
        mmu.memory[0x0000] = 0xD3;
        mmu.memory[0x0001] = 0x00;
        // inject "out 1,a" at 0x0005 (signal to output some characters)
        mmu.memory[0x0005] = 0xD3;
        mmu.memory[0x0006] = 0x01;
        mmu.memory[0x0007] = 0xC9;

        mmu
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    pub fn get_vram(&self) -> &[u8] {
        &self.memory[0x2400..0x4000]
    }
}

fn read_complete_rom(rom_path: &str) -> io::Result<(Vec<u8>, usize)> {
    let mut f = File::open(rom_path)?;
    let mut buffer = Vec::new();
    let size = f.read_to_end(&mut buffer)?;
    Ok((buffer, size))
}
