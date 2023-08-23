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
	@echo "Run 'make install' to install CodingFetch"

install:
	@echo "Detected $(OS), installing to $(INSTALL_BIN)"
	@echo "Todo"

uninstall:
	@echo "Currently installed at $(INSTALL_BIN)"
	@echo "Todo"
