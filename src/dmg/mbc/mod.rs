mod no_mbc;

use no_mbc::NO_MBC;

use std::path::PathBuf;

pub trait MBC {
    // Constructor
    fn new() -> Box<dyn MBC> where Self: Sized;
    
    // Getters
    fn cartridge(&self) -> &Vec<u8>;

    // Method
    fn load_cartridge(&mut self, cartridge_path: PathBuf);
    
    // Default method definition
    fn print_cartridge_title(&self) {
        for i in 0x134..0x143 {
            print!("{}", self.cartridge()[i] as char);
        }
        println!();
    }
}

impl Default for Box<dyn MBC> {
    fn default() -> Box<dyn MBC> {
        NO_MBC::new()
    }
}