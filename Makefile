# Compiler
CARGO = cargo

# Directories
BUILD_DIR = target/release

all: clean build

build: 
	$(CARGO) build --release

clean:
	$(CARGO) clean

.PHONY: all clean
