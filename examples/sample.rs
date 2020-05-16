extern crate elf;

use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("./test1");
    let file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };
    
    let text_scn = match file.get_section(".text") {
        Some(s) => s,
        None => panic!("Failed to look up .text section"),
    };
    
    println!("{:?}", text_scn.data);
}
