pub mod mbc_type;

use super::MMU;

pub trait MBC {
    // Method
    fn update_banks(&self, mmu: &mut MMU, cartridge: &Vec<u8>);
    
    // Default method definition
    fn print_cartridge_title(&self, cartridge: &Vec<u8>) {
        for i in 0x134..0x143 {
            print!("{}", cartridge[i] as char);
        }
        println!();
    }
}