/*    Main entry
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






mod ElfGaspard;
use std::env;
use crate::ElfGaspard::gaspard_elf;

fn main() {

println!( "Remul  Copyright (C) 2024  Gaspard COURCHINOUX \n
This program comes with ABSOLUTELY NO WARRANTY;\n
This is free software, and you are welcome to redistribute it \n
under certain conditions; ");


let args : Vec<String> = env::args().collect();
gaspard_elf(args);

}//
