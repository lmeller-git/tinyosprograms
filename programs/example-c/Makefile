RUST_LIB_PATH ?= "includes/libtinyos/target/target/release/liblibtinyos.a"
LIBGCC_PATH ?= "/usr/lib/gcc/x86_64-linux-gnu/12/libgcc.a"
LINKER_PATH ?= ./linker.ld
SRC ?= src/main.c
INCLUDES ?= -I ./includes

CFLAGS = -nostdlib -nostartfiles -nodefaultlibs -ffreestanding -static \
		 -s -T $(LINKER_PATH) -no-pie -Wl,--build-id=none -Wl,--strip-all \
		 $(RUST_LIB_PATH) $(LIBGCC_PATH) $(INCLUDES)

.PHONY: all build clean libtinyos

all: libtinyos build

build:
	$(MAKE) libtinyos
	gcc $(SRC) $(CFLAGS) -o a.out

libtinyos:
	mkdir -p includes
	if [ ! -d includes/libtinyos ]; then \
		echo "cloning libtinyos" \
		git clone https://github.com/lmeller-git/libtinyos includes/libtinyos; \
	else \
		cd includes/libtinyos && git pull; \
	fi
	cd includes/libtinyos && cargo build --release && mv libtinyos/libtinyos.h ..
	
clean:
	rm a.out
	rm -rf includes/

