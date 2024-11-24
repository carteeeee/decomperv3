use std::fs;

mod levelscript;

fn main() {
    println!("Hello, world!");
    let rom = fs::read("rom.z64").expect("could not read the rom"); // the filename is
    levelscript::parse_entry(rom, 0x2ABCA0);
}
