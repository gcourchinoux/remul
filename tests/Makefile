CC=riscv32-unknown-elf-gcc
CFLAGS=-nostdlib -g

OBJECTS = test1.o



test1.o: test1.c
	$(CC) $(CFLAGS) $^ -o $@


clean:
	rm -rf *.o 
