RUST_LIB_PATH ?= "includes/libtinyos/liblibtinyos.a"
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
		echo "downloading latest libtinyos release"; \
		mkdir -p includes/libtinyos; \
		curl -L https://github.com/lmeller-git/libtinyos/releases/download/v0.0.0/libtinyos.a -o includes/libtinyos/libtinyos.a; \
		curl -L https://github.com/lmeller-git/libtinyos/releases/download/v0.0.0/libtinyos.h -o includes/libtinyos.h; \  
	else \
		echo "cleaning up includes"; \
		rm -rf includes/; \
		echo "downloading latest libtinyos release"; \
		curl -L https://github.com/lmeller-git/libtinyos/releases/download/v0.0.0/libtinyos.a -o includes/libtinyos/libtinyos.a; \
		curl -L https://github.com/lmeller-git/libtinyos/releases/download/v0.0.0/libtinyos.h -o includes/libtinyos.h; \  
	fi
	echo "downloaded latest libtinyos release"
	
clean:
	rm a.out
	rm -rf includes/

