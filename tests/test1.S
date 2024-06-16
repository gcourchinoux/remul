
 
/*    Output : test1.o.elf
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
.global main
.global jump


jump:


    add x2,x1,x4
    sub x2,x1,x4
    xor x2,x1,x4
    or x2,x1,x4
    and x2,x1,x4
    srl x2,x1,x4
    sra x2,x1,x4
    ret


main:
addi x2,x2,50
    add x2,x1,x4
    sub x2,x1,x4
    xor x2,x1,x4
    or x2,x1,x4
    and x2,x1,x4
    srl x2,x1,x4
    sra x2,x1,x4
    jal jump
    la x1,jump
    JALR x2,x1,0    
    addi x3,x3,50
    addi x10,x10,50
    beq x3,x10,jump
    addi x3,x3,5
    bne x3,x10,jump