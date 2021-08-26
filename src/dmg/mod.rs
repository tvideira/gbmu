mod cpu;
mod mmu;
mod ppu;
mod mbc;

extern crate minifb;

use cpu::CPU;
use mmu::MMU;
use ppu::PPU;
use mbc::MBC;
use mbc::get_mbc;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use minifb::{Key, Scale, Window, WindowOptions};

// GAME BOY STRUCT
#[derive(Default)]
pub struct DMG {
    cpu: CPU,
    mmu: MMU,
    ppu: PPU,
    mbc: MBC,
    cartridge: Vec<u8>,
    clock: u32,
}

impl DMG {
    pub fn load_cartridge(&mut self, cartridge_path: PathBuf) {
        let mut file = match File::open(cartridge_path) {
            Ok(file) => file,
            Err(why) => panic!("couldn't open file because: {}", why),
        };
    
        let _size = match file.read_to_end(&mut self.cartridge) {
            Ok(_size) => _size,
            Err(why) => panic!("couldn't read file because: {}", why),
        };

        self.mbc = get_mbc(&self.cartridge);
        println!("{}", self.mbc);

        self.mmu.disable_bios();
        self.mbc.update_banks(&mut self.mmu, &self.cartridge);
        self.mmu.enable_bios();
    }

    fn print_cartridge_title(&self) {
        for i in 0x134..0x143 {
            print!("{}", self.cartridge[i] as char);
        }
        println!();
    }

    pub fn start(&mut self) {
        self.print_cartridge_title();
        self.run();
    }

    fn run(&mut self) {
        let mut window = match Window::new("gbmu", 160, 144, WindowOptions { resize: false, scale: Scale::X2, ..WindowOptions::default()}) {
            Ok(win) => win,
            Err(err) => {
                println!("Unable to create window {}", err);
                return;
            }
        };
    
        let mut window_2 = match Window::new("tile_set", 128, 384, WindowOptions { resize: false, scale: Scale::X2, ..WindowOptions::default()}) {
            Ok(win) => win,
            Err(err) => {
                println!("Unable to create window {}", err);
                return;
            }
        };
    
        let mut window_3 = match Window::new("background", 256, 256, WindowOptions { resize: false, scale: Scale::X2, ..WindowOptions::default()}) {
            Ok(win) => win,
            Err(err) => {
                println!("Unable to create window {}", err);
                return;
            }
        };

        window.set_position(500, 20);
        window_2.set_position(20, 20);
        window_3.set_position(1000, 20);
        
        let mut i = 0;

        let now = SystemTime::now();

        while window.is_open() && !window.is_key_down(Key::Escape) && window_2.is_open() && !window_2.is_key_down(Key::Escape)  && window_3.is_open() && !window_3.is_key_down(Key::Escape) {
            i += 1;
            println!("FRAME {}", i);
            self.frame(&mut window, &mut window_2, &mut window_3);
            match now.elapsed() {
                Ok(elapsed) => {
                    println!("{}", elapsed.as_millis());
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
    }

    fn step(&mut self) {
        let delta_clock = self.cpu.step(&mut self.mmu);
        self.clock += delta_clock;
        self.ppu.step(&mut self.mmu, delta_clock);
    }

    fn frame(&mut self, main_window: &mut Window, tile_data_window: &mut Window, background_window: &mut Window) {
        let cycle = self.clock + 70224;
        while self.clock < cycle {
            self.step();
        }
        main_window.update_with_buffer(&self.ppu.main_screen_buffer, 160, 144).unwrap();
        tile_data_window.update_with_buffer(&self.ppu.tile_data_buffer, 128, 384).unwrap();
        background_window.update_with_buffer(&self.ppu.background_buffer, 256, 256).unwrap();
    }
}