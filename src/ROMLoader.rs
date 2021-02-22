use std::fs::File;
use std::io::prelude::*;

pub struct ROMLoader {
    pub rom: [u8; 128],
}

pub fn new(rom_name: &str) -> ROMLoader {

    let mut rom = File::open(rom_name).expect("ROM not found");
    let mut data = [0u8; 128];
    let size = rom.read(&mut data).expect("ROM cannot be read");

    ROMLoader {
        rom: data,
    }


}
