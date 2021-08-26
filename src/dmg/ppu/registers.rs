#[derive(Default)]
pub struct LCDC {
    pub lcd_display_enable: bool,
    pub window_tile_map_address: bool,
    pub window_enable: bool,
    pub bg_window_tile_data: bool,
    pub bg_tile_map_address: bool,
    pub obj_size: bool,
    pub obj_enable: bool,
    pub bg_enable: bool,
    pub val: u8,
}

impl LCDC {
    
}

#[derive(Default)]
pub struct STAT {
    pub lyc_lc_interrupt: bool,
    pub mode_2_oam_interrupt: bool,
    pub mode_1_vblank_interrupt: bool,
    pub mode_0_hblank_interrupt: bool,
    pub lyc_ly_flag: bool,
    pub mode: u8,
    pub val: u8,
}

impl STAT {
    pub fn set_mode(&mut self, val: u8){
        self.mode = val;
        self.val = (self.val & 0xFC) | val;
    }
}

#[derive(Default)]
pub struct REGISTERS {
    pub lcdc: LCDC,
    pub stat: STAT,
    pub scy: u8,
    pub scx: u8,
    pub ly: u8,
    pub lyc: u8,
    pub dma: u8,
    pub bgp: u8,
    pub obp_0: u8,
    pub obp_1: u8,
    pub wy: u8,
    pub wx: u8,
}

impl REGISTERS {
    pub fn set_lcdc(&mut self, val: u8){
        self.lcdc.lcd_display_enable        = (val & 0x80) == 0x80;
        self.lcdc.window_tile_map_address   = (val & 0x40) == 0x40;
        self.lcdc.window_enable             = (val & 0x20) == 0x20;
        self.lcdc.bg_window_tile_data       = (val & 0x10) == 0x10;
        self.lcdc.bg_tile_map_address       = (val & 0x08) == 0x08;
        self.lcdc.obj_size                  = (val & 0x04) == 0x04;
        self.lcdc.obj_enable                = (val & 0x02) == 0x02;
        self.lcdc.bg_enable                 = (val & 0x01) == 1;
        self.lcdc.val = val;
    }

    pub fn set_stat(&mut self, val: u8){
        self.stat.lyc_lc_interrupt          = (val & 0x40) == 1;
        self.stat.mode_2_oam_interrupt      = (val & 0x20) == 1;
        self.stat.mode_1_vblank_interrupt   = (val & 0x10) == 1;
        self.stat.mode_0_hblank_interrupt   = (val & 0x08) == 1;
        self.stat.lyc_ly_flag               = (val & 0x04) == 1;
        self.stat.mode                      = val & 0x03;
        self.stat.val = val;
    }

    pub fn get_lcdc(&mut self) -> u8 {
        return self.lcdc.val;
    }

    pub fn get_stat(&mut self) -> u8{
        
        return self.stat.val;
    }

    pub fn debug(&self) {
        println!("DEBUG PPU REGISTERS :");
        println!("lcdc:  0x{:02X} | stat:  0x{:02X}", self.lcdc.val, self.stat.val);
        println!("scy:   0x{:02X} | scx:   0x{:02X}", self.scy, self.scx);
        println!("ly:    0x{:02X} | lyc:   0x{:02X}", self.ly, self.lyc);
        println!("dma:   0x{:02X} | bgp:   0x{:02X}", self.dma, self.bgp);
        println!("opb_0: 0x{:02X} | opb_1: 0x{:02X}", self.obp_0, self.obp_1);
        println!("wy:    0x{:02X} | wx:    0x{:02X}", self.wy, self.wx);
    }
}