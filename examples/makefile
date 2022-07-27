CC=riscv64-unknown-elf-gcc

all:
	${CC} -Wl,-Ttext=0x00000000 -nostdlib -march=rv64i -mabi=lp64 fib.c -o fib.o
	riscv64-unknown-elf-objcopy -O binary  fib.o fib.bin 
	${CC} -Wl,-Ttext=0x00000000 -nostdlib -march=rv64i -mabi=lp64 helloworld.c -o helloworld.o
	riscv64-unknown-elf-objcopy -O binary  helloworld.o helloworld.bin 

clean:
	rm -f fib.o fib.bin
	rm -f helloworld.o helloworld.bin
