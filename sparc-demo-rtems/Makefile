CC=sparc-gaisler-rtems5-clang
OBJDUMP=sparc-gaisler-rtems5-objdump
CFLAGS=-mcpu=leon3 \
	-qbsp=leon3 \
	-Os \

all: rtems-rust

rebuild: clean all

run: rtems-rust
	tsim-leon3 -c sim-commands.txt ./rtems-rust

rtems-rust: rtems-rust.c rtemsdemo/target/sparc-unknown-none-elf/debug/librtemsdemo.a

rtemsdemo/target/sparc-unknown-none-elf/debug/librtemsdemo.a: rtemsdemo/src/lib.rs
	cd ./rtemsdemo && cargo +nightly build --target=sparc-unknown-none-elf

rtems-rust.asm: rtems-rust
	$(OBJDUMP) -td ./rtems-rust > ./rtems-rust.asm

clean:
	rm -rf rtems-rust rtems-rust.asm
	cd ./rtemsdemo && cargo +nightly clean

.PHONY: all clean rebuild run
