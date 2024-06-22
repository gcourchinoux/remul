# hardcoder un fichier elf dans un array rust 
import sys 

f = open(sys.argv[1],"rb")




f = f.read()

str_ = f"let file_elf[u8,{len(f)}] = ["

for x in f:
    str_ += f"{str(hex(x))}, \n"

str_ += "];"

file = open("output.rs","w")


file.write(str_)
file.close()



print(str_)


