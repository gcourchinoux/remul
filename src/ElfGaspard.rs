/*    ELF file
    Copyright (C) 2024  Gaspard COURCHINOUX

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/


use elf::ElfBytes;
use elf::endian::AnyEndian;
use elf::note::Note;
use elf::note::NoteGnuBuildId;
use elf::section::SectionHeader;

use std::any::type_name;
mod Decoder;
use crate::ElfGaspard::Decoder::DecoderStruct;

use wasm_bindgen::prelude::*;



#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}



/// adress = adresse de démarrage
fn open_data(adress: u64,data: &[u8],sh_addr:u64) {


let mut vec = Vec::new();


let mut decoder = DecoderStruct::new(adress,data,vec,sh_addr);

decoder.init()
}

fn open_elf(file : &str) {

let path = std::path::PathBuf::from(file);
let file_data = std::fs::read(path).expect("Could not read file.");
let slice = file_data.as_slice();
let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("Open test1");

// Get the ELF file's build-id
let text: SectionHeader = file
    .section_header_by_name(".text")
    .expect("section table should be parseable")
    .expect("file should have a .note.ABI-tag section");


let data = file.section_data(&text).unwrap();

// vérifier que c'est bien riscv 


let adress = file.ehdr.e_entry;
println!("adresse de demarrage {:x}",adress);

let (data,option) = data;
open_data(adress,data,text.sh_addr)


}
pub fn open_elf_web(file : &[u8]) {

    console_log!("file pas encore scanner {:x} ",file[0]);

    let filex = ElfBytes::<AnyEndian>::minimal_parse(file).expect("Open test1");
    console_log!("fichier ok ");

    // Get the ELF file's build-id
    let text: SectionHeader = filex
        .section_header_by_name(".text")
        .expect("section table should be parseable")
        .expect("file should have a .note.ABI-tag section");
    
    
    let data = filex.section_data(&text).unwrap();
    
    // vérifier que c'est bien riscv 
    
    
    let adress = filex.ehdr.e_entry;
   // console_log!("adresse de demarrage {:x}",adress);
    
    let (data,option) = data;
    open_data(adress,data,text.sh_addr)
    
    
    }
    

pub fn gaspard_elf(argv: Vec<String>) {

if argv.len() > 1 {


let file = &argv[1];
println!("Fichier ELf {}",file);


open_elf(file);


} else {




println!("Not enough argv");

}

}
