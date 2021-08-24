use super::MBC;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Default)]
pub struct NO_MBC {
    cartridge: Vec<u8>,
}

impl MBC for NO_MBC {
    fn new() -> Box<dyn MBC> {
        Box::new(NO_MBC{ cartridge: Vec::default(), })
    }

    fn cartridge(&self) -> &Vec<u8> {
        &self.cartridge
    }
    
    fn load_cartridge(&mut self, cartridge_path: PathBuf) {
        let mut file = match File::open(cartridge_path) {
            Ok(file) => file,
            Err(why) => panic!("couldn't open file because: {}", why),
        };
    
        let _size = match file.read_to_end(&mut self.cartridge) {
            Ok(_size) => _size,
            Err(why) => panic!("couldn't read file because: {}", why),
        };
    }
}