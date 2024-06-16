# Démarrer rapidement.



```
cargo run test.o.elf # simple program , it mades some test 
```

# Présentation. 


Je suis Gaspard COURCHINOUX , je suis un (ancien) étudiant du M2 droit du travail et de la protection sociale, je suis passionné d'informatique. 


J'ai réalisé cet émulateur pour apprendre le jeu d'instructions (ISA) RISCV32. 


Je pense que le futur c'est RISCV et je trouve cette architecture très séduisante notamment par sa simplicité. 


Le code que vous lisez est écrit en RUST avec peu de dépendances. 


Il n'est pas parfait :  j'essaie de faire un code fonctionnel et lisible pour le plus grand nombre.



La licence est la licence publique générale gnu v3. 

Vous pouvez la lire en ouvrant le fichier gpl-3.0.md

Si vous avez des questions, n'hésitez pas à m'écrire : gaspard.courchinoux@protonmail.ch



# Flux de l'information. 


Remul32 est un émulateur qui a été construit dans l'optique d'être lisible et facilement débugable. 

Remul32 prend en entrée (input) un fichier ELF (executable linkable format) compilé au prélable avec un compilateur CLANG/LLVM ou GCC/BINUTILS. 


Les tests ont été faits sur GCC/BINUTILS principalement. Même si CLANG llvm est un compilateur plus moderne et plus sécurisé. 

Le fichier est ouvert, la section : .TEXT est décompilé. Aussi, les instructions sont copiées dans une matrice (Vec) qui sert à représenter la mémoire. 


Elles sont copiées à partir de l'adresse de démarrage que fourni le fichier ELF. 


# Gestion de la mémoire.


Remul32 se veut être léger, il ne demande pas une allocation continue d'un gros bloc de mémoire. 


La mémoire est allouée en fonction des opérations de  LOAD/STORE, si le programme copie 300 octets : 300 octets seront alors alloués. 


# Instructions prises en charge.

Fichier Decoder.RS

Toutes les instructions RISCV (~1426, en incluant les extensions sont prises en charge.). 

Bien sur , je suis seul, je ne peux pas écrire le code d'émulation 1426 instructions. Cela serait trop long. 

Si vous souhaitez contribuer (la majeure partie du travail consiste à ajouter des instructions.), n'hésitez pas a cloner ce dépot et à faire une PULL request. Je serai très honoré de votre dévouement !


Les instructions prises en charge sont RV32I. Néanmmoins, toutes les instructions sont prises en compte. Celle que je n'ai pas implémentées sont ignorées simplement. 


RVC est détecté, mais ignoré pour l'instant. 

# Interne du programme.



Le fichier Decoder.RS a été partiellement généré à l'aide RISCV-OPCODES qui est un programme python fourni par la fondation RISCV. 

Il permet de générer des fichiers rust,c etc. pour décompiler. 

J'ai modifié le programme python pour générer un fichier RUST conforme à mes attentes. 


Le fichier Decoder.rs est un fichier central, le plus important du programme. 


Ce fichier implemente une structure qui est DecoderStruct. 

Elle implémente la matrice contenant la mémoire. 

```

pub struct DecoderStruct{


adress_begin :u64,

adress_instructions : &'a [u8],




memory: Vec,

section_instr_size: u32,
regs: [u64; 33],

// faire les drapeaux




carry : bool,
negative: bool,
zero : bool,
equal : bool ,
overflow: bool ,
greater_than : bool,
greater_than_equal : bool,
less_than_equal : bool,
less_than : bool,


adress_debut_fonction : u64,
adress_begin_section : u64,


}

```



Pour implémenter une nouvelle instruction, vous devez impérativement écrire le code dans normal_scan. 

La variable self contient la structure DecoderStruct. Vous devez faire l'action de l'instruction à l'aide de cette structure. 




# la suite.



Il faut rajouter d'autres instructions et corriger des bugs d'execution notamment dans les instructions beq etc et JALR, JAL. 



# Pour me contacter. 

N'hésitez pas si vous avez des questions.


gaspard.courchinoux@protonmail.ch


https://courchinoux.org


