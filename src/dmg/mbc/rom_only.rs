use super::MMU;

pub fn update_banks_rom_only(mmu: &mut MMU, cartridge: &Vec<u8>) {
    for i in 0..cartridge.len() {
        mmu.write_byte(i as u16, cartridge[i]);
    }
}