CC=sparc-gaisler-elf-clang
CFLAGS=-mcpu=leon3 -Os

all: test1

rebuild: clean all

run: test1
	tsim-leon3 -c sim-commands.txt ./test1

test1: test1.c

test1.asm: test1
	sparc-gaisler-elf-objdump -td ./test1 > ./test1.asm

clean:
	@rm -rf test1 test1.asm

.PHONY: all clean rebuild run
