UNAME := $(shell uname -s)
OS = "Windows" # Default if windows doesn't have uname
INSTALL_DIR = "Not yet defined"
ifeq ($(UNAME), Linux)
OS = "Linux"
INSTALL_DIR = /usr/local/bin
endif
ifeq ($(UNAME), Darwin)
OS = "MacOS"
INSTALL_DIR = "Not yet defined"
endif
all:
	@echo "Run 'make build' to build binary before installing"

build:
	@cargo build --release
	@echo "Build complete"
	@echo "Run 'make install' to install CodingFetch"

install:
	@echo "Detected $(OS), installing to $(INSTALL_DIR)"
	@cp ./target/release/codingfetch /usr/bin/codingfetch
	@chmod 755 /usr/bin/codingfetch


uninstall:
	@echo "Currently installed at $(INSTALL_DIR)"
	@rm -rf /usr/bin/codingfetch
