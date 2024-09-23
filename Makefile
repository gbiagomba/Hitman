# Variables
BINARY=hitman
SRC_DIR=src
BUILD_DIR=target/release
INSTALL_DIR=/usr/local/bin

# Build the release binary
build:
	cargo build --release

# Install the binary to /usr/local/bin
install: build
	install -m 755 $(BUILD_DIR)/$(BINARY) $(INSTALL_DIR)/$(BINARY)

# Run the binary
run:
	$(BUILD_DIR)/$(BINARY)

# Clean the build artifacts
clean:
	cargo clean

# Test the code
test:
	cargo test
