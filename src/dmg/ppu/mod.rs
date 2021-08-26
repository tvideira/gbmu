mod registers;

use registers::REGISTERS;

use super::MMU;

const RED: u32 = 0xFF0000;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

// PIXEL PROCESSING UNIT STRUCT
pub struct PPU {
    clock: u32,
    registers: REGISTERS,
    pub main_screen_buffer: Vec<u32>,
    pub tile_data_buffer: Vec<u32>,
    pub background_buffer: Vec<u32>,
    colors: Vec<u32>,
}

impl Default for PPU {
    fn default() -> Self {
        Self {
            clock: 0,
            registers: REGISTERS::default(),
            main_screen_buffer: vec![0; 160 * 144],
            tile_data_buffer: vec![0; 128 * 384],
            background_buffer: vec![0; 256 * 256],
            colors: vec![
                from_u8_rgb(0xE0, 0xF8, 0xD0),
                from_u8_rgb(0x88, 0xC0, 0x70),
                from_u8_rgb(0x34, 0x68, 0x56),
                from_u8_rgb(0x08, 0x18, 0x20),
            ],
        }
    }
}

impl PPU {
    pub fn step(&mut self, mmu: &mut MMU, delta_clock: u32) {
        self.clock += delta_clock;

        let ly = mmu.read_byte(0xFF44) ;
        let stat = mmu.read_byte(0xFF41);
        let mode = stat & 0x03;
        let stat = stat & 0xFC;

        if self.clock >= 456 {
            self.clock -= 456;
            mmu.write_byte(0xFF44, (ly + 1) % 154);

            if ly >= 144 && mode != 1 {
                mmu.write_byte(0xFF41, stat | 1);
                //self.render_tile_set(mmu);
                self.render_background(mmu);
            }
        }
        if ly < 144 {
            if self.clock <= 20 {
                if mode != 2 {
                    mmu.write_byte(0xFF41, stat | 2);
                }
            }
            else if self.clock <= (20 + 43) {
                if mode != 3 {
                    mmu.write_byte(0xFF41, stat | 3);
                }
            }
            else {
                if mode != 0 {
                    mmu.write_byte(0xFF41, stat);
                    self.render_scan(mmu);
                }
            }
        }
    }

    pub fn render_tile_set(&mut self, mmu: & MMU) {
        let bgp = mmu.read_byte(0xFF47);
        let palette = self.make_palette(bgp);
        
        for i in 0..384 {
            let x: usize = i % 16;
            let y: usize = i / 16;
            let tile = mmu.get_tile_data(i);

            for tile_y in 0..8 {
                for tile_x in 0..8 {
                    let pixel = self.get_tile_pixel(&tile, tile_x, tile_y);
                    let idk = palette[pixel as usize];
                    let color = self.colors[idk as usize];
                    self.tile_data_buffer[(y * 8 * 128 + tile_y as usize * 128) + (x * 8) + tile_x as usize] = color;
                }
            }
        }
    }

    pub fn render_background(&mut self, mmu: &MMU) {
        let bg_tile_map_address = (mmu.read_byte(0xFF40) & 0x08) == 0x08;
        let bg_window_tile_data = (mmu.read_byte(0xFF40) & 0x10) == 0x10;
        let scy = mmu.read_byte(0xFF42);
        let scx = mmu.read_byte(0xFF43);
        let bgp = mmu.read_byte(0xFF47);

        let palette = self.make_palette(bgp);

        let base_addr: u16 = if bg_tile_map_address { 0x9C00 } else { 0x9800 };        
        for y in 0..32 {
            for x in 0..32 {
                let coordinate = base_addr + (y * 32 + x) as u16;
                
                let mut tile_index = mmu.read_byte(coordinate) as usize;
                
                if !bg_window_tile_data && tile_index < 128 {
                   tile_index += 256;
                }

                let tile = mmu.get_tile_data(tile_index);
                
                for tile_y in 0..8 {
                    for tile_x in 0..8 {
                        let pixel = self.get_tile_pixel(&tile, tile_x, tile_y);
                        let idk = palette[pixel as usize];
                        let color = self.colors[idk as usize];
                        self.background_buffer[(y * 8 * 256 + tile_y as usize * 256) + (x * 8) + tile_x as usize] = color;
                    }
                }
            }
        }
        
        let y = scy as usize;
        let y_2 = y + 144 % 256;
        let off_x = scx as usize;
        for x in 0..160 {
            self.background_buffer[(y * 256) + (x as usize) +  off_x] = RED;
            self.background_buffer[(y_2 * 256) + (x as usize) +  off_x] = RED;
        }
        if y < y_2 {
            for i in y..y_2 {
                self.background_buffer[(i * 256) + off_x] = RED;
                self.background_buffer[(i * 256) + off_x + 160] = RED;
            }       
        } else {
            for i in 0..y_2 {
                self.background_buffer[(i * 256) + off_x] = RED;
                self.background_buffer[(i * 256) + off_x + 160] = RED;
            }

            for i in y..256 {
                self.background_buffer[(i * 256) + off_x] = RED;
                self.background_buffer[(i * 256) + off_x + 160] = RED;
            }
        }
    }

    pub fn render_scan(&mut self, mmu: & MMU) {
        // which screen line will be written
        let y = mmu.read_byte(0xFF44) as usize;
        let scy = mmu.read_byte(0xFF42) as usize;
        let scx = mmu.read_byte(0xFF43) as u16;
        let bg_tile_map_address = (mmu.read_byte(0xFF40) & 0x08) == 0x08;
        let bg_window_tile_data = (mmu.read_byte(0xFF40) & 0x10) == 0x10;
        let bgp = mmu.read_byte(0xFF47);

        // which line to render (LY + SCY % 256)
        let start_y: u16 = ((y + scy) as u16) & 0xFF;

        // FIND TILE_MAP_ADDR
        let mut tile_map_addr: u16 = if bg_tile_map_address { 0x9C00 } else { 0x9800 };
        tile_map_addr += 32 * (start_y >> 3); // offset the line part
        
        let mut tile_map_addr_off_x = scx >> 3; // offset the column part
        
        // FIND WHICH TILE TO DISPLAY
        let mut tile_index = mmu.read_byte(tile_map_addr + tile_map_addr_off_x) as usize;
        if !bg_window_tile_data && tile_index < 128 { tile_index += 256; }

        let mut tile_x = (scx % 8) as u8;
        let tile_y = (start_y % 8) as u8;

        let mut tile = mmu.get_tile_data(tile_index);

        let palette = self.make_palette(bgp);
        for x in 0 .. 160 {
            let pixel = self.get_tile_pixel(&tile, tile_x, tile_y);
            self.main_screen_buffer[y * 160 + x] = self.colors[palette[pixel as usize] as usize];

            tile_x += 1;
            if tile_x >= 8 {
                tile_x = 0;
                tile_map_addr_off_x = (tile_map_addr_off_x + 1) % 32;
                tile_index = mmu.read_byte(tile_map_addr + tile_map_addr_off_x) as usize;
                
                if !bg_window_tile_data && tile_index < 128 { tile_index += 256; }
                
                tile = mmu.get_tile_data(tile_index);
            }
        }
    }

    pub fn get_tile_pixel(&self, tile: & Vec<u8>, x: u8, y: u8) -> u8 {
        let byte_0 = tile[(2 * y) as usize];
        let byte_1 = tile[(2 * y + 1) as usize];

        let val_0 = (byte_0 >> (7 - x)) & 0x01;
        let val_1 = (byte_1 >> (7 - x)) & 0x01;

        return (val_1 << 1) | val_0;
    }

    pub fn make_palette(&self, register: u8) -> Vec<u8> {
        let mut palette: Vec<u8> = vec!(0; 4);
        palette[0] = register & 0x03;
        palette[1] = (register & 0x0C) >> 2;
        palette[2] = (register & 0x30) >> 4;
        palette[3] = (register & 0xC0) >> 6;
        return palette;
    }

    pub fn update_registers(&mut self, mmu: & MMU) {
        self.registers.set_lcdc(mmu.read_byte(0xFF40));
        self.registers.set_stat(mmu.read_byte(0xFF41));
        self.registers.scy      = mmu.read_byte(0xFF42);
        self.registers.scx      = mmu.read_byte(0xFF43);
        self.registers.ly       = mmu.read_byte(0xFF44);
        self.registers.lyc      = mmu.read_byte(0xFF45);
        self.registers.dma      = mmu.read_byte(0xFF46);
        self.registers.bgp      = mmu.read_byte(0xFF47);
        self.registers.obp_0    = mmu.read_byte(0xFF48);
        self.registers.obp_1    = mmu.read_byte(0xFF49);
        self.registers.wy       = mmu.read_byte(0xFF4A);
        self.registers.wx       = mmu.read_byte(0xFF4B);
    }

    pub fn update_mmu(&mut self, mmu: &mut MMU) {
        mmu.write_byte(0xFF40, self.registers.get_lcdc());
        mmu.write_byte(0xFF41, self.registers.get_stat());
        mmu.write_byte(0xFF42, self.registers.scy);
        mmu.write_byte(0xFF43, self.registers.scx);
        mmu.write_byte(0xFF44, self.registers.ly);
        mmu.write_byte(0xFF45, self.registers.lyc);
        mmu.write_byte(0xFF46, self.registers.dma);
        mmu.write_byte(0xFF47, self.registers.bgp);
        mmu.write_byte(0xFF48, self.registers.obp_0);
        mmu.write_byte(0xFF49, self.registers.obp_1);
        mmu.write_byte(0xFF4A, self.registers.wy);
        mmu.write_byte(0xFF4B, self.registers.wx);
    }

    pub fn debug(&self) {
        self.registers.debug();
    }
}