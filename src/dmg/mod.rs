mod cpu;
mod mmu;
mod ppu;
mod mbc;

extern crate minifb;

use cpu::CPU;
use mmu::MMU;
use ppu::PPU;

use std::path::PathBuf;

use minifb::{Key, Scale, Window, WindowOptions};

// GAME BOY STRUCT
#[derive(Default)]
pub struct DMG {
    cpu: CPU,
    mmu: MMU,
    ppu: PPU,
    clock: u32,
    exec_bios: bool,
    debug: bool,
}

impl DMG {
    pub fn load_cartridge(&mut self, cartridge_path: PathBuf) {
        self.mmu.load_cartridge(cartridge_path);
    }

    pub fn start(&mut self) {
        self.exec_bios = true;
        self.debug = false;
        if !self.exec_bios {
            self.skip_bios();
        }
        self.run();
    }

    fn run(&mut self) {
        let mut main_window = match Window::new("gbmu", 160, 144, WindowOptions { resize: false, scale: Scale::X2, ..WindowOptions::default()}) {
            Ok(win) => win,
            Err(err) => {
                println!("Unable to create window {}", err);
                return;
            }
        };
    
        let mut tile_data_window = match Window::new("tile_set", 128, 384, WindowOptions { resize: false, scale: Scale::X2, ..WindowOptions::default()}) {
            Ok(win) => win,
            Err(err) => {
                println!("Unable to create window {}", err);
                return;
            }
        };
    
        let mut background_window = match Window::new("background", 256, 256, WindowOptions { resize: false, scale: Scale::X2, ..WindowOptions::default()}) {
            Ok(win) => win,
            Err(err) => {
                println!("Unable to create window {}", err);
                return;
            }
        };

        main_window.set_position(500, 20);
        tile_data_window.set_position(20, 20);
        background_window.set_position(1000, 20);
        
        let mut i = 0;

        while main_window.is_open() && !main_window.is_key_down(Key::Escape) && tile_data_window.is_open() && !tile_data_window.is_key_down(Key::Escape)  && background_window.is_open() && !background_window.is_key_down(Key::Escape) {
            main_window.get_keys().map(|keys| {
                for t in keys {
                    match t {
                        //Key::Up => core.mmu.keys[1] &= 0b1011,
                        //Key::Down => core.mmu.keys[1] &= 0b0111,
                        //Key::Left => core.mmu.keys[1] &= 0b1101,
                        //Key::Right => core.mmu.keys[1] &= 0b1110,
                        //Key::Z => core.mmu.keys[0] &= 0b1101, //B
                        //Key::X => core.mmu.keys[0] &= 0b1110, //A
                        //Key::Apostrophe => core.mmu.keys[0] &= 0b1011, //Select
                        //Key::Enter => core.mmu.keys[0] &= 0b0111, //Start
                        Key::N => self.debug = !self.debug,
                        _ => (),
                    }
                }
            });

            i += 1;
            println!("FRAME {}", i);
            // RUN FRAME
            self.frame();
            
            // UPDATE WINDOW
            main_window.update_with_buffer(&self.ppu.main_screen_buffer, 160, 144).unwrap();
            tile_data_window.update_with_buffer(&self.ppu.tile_data_buffer, 128, 384).unwrap();
            background_window.update_with_buffer(&self.ppu.background_buffer, 256, 256).unwrap();
        }
    }

    fn frame(&mut self) {
        let cycle = self.clock + 70224;
        while self.clock < cycle {
            self.step();
        }
    }

    fn step(&mut self) {
        let delta_clock = self.cpu.step(&mut self.mmu, self.debug);
        self.clock += delta_clock;
        self.ppu.step(&mut self.mmu, delta_clock);
        self.update(delta_clock);
    }

    fn update(&mut self, delta_clock: u32) {
        let tac_frequency_tab = [1024, 16, 64, 256];

        let ff05 = self.mmu.read_byte(0xFF05); // TIMA TIMER COUNTER
        let ff07 = self.mmu.read_byte(0xFF07); // TAC TIMER CONTROLLER
        if (ff07 & 0x04) == 0x04 {
            let tac_frequency = tac_frequency_tab[(ff07 & 0x03) as usize];
            if (self.clock % tac_frequency) < delta_clock {
                if ff05 == 0xFF {
                    self.mmu.write_byte(0xFF05, self.mmu.read_byte(0xFF06));
                    self.mmu.write_byte(0xFF0F, self.mmu.read_byte(0xFF0F) | 0x04);
                } else {
                    self.mmu.write_byte(0xFF05, ff05.wrapping_add(1));
                }
            }
        }
    }

    fn skip_bios(&mut self) {
        self.mmu.skip_bios();
        self.cpu.skip_bios();
    }
}