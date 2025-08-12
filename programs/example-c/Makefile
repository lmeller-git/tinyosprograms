RUST_LIB_PATH ?= "includes/libtinyos/libtinyos.a"
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
	if [ ! -e includes/libtinyos/libtinyos.a ] || [ ! -e includes/libtinyos.h ]; then \
		echo "downloading latest libtinyos release"; \
		mkdir -p includes/libtinyos; \
		curl -L https://github.com/lmeller-git/libtinyos/releases/download/v0.0.0/libtinyos.a -o includes/libtinyos/libtinyos.a; \
		curl -L https://github.com/lmeller-git/libtinyos/releases/download/v0.0.0/libtinyos.h -o includes/libtinyos.h; \
	fi

	echo "libtinyos in includes/"

clean:
	rm -f a.out
	rm -rf includes/

