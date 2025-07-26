# Upload Workflow - ESP32 Rust Development

## Prerequisites Setup

### 1. Install Rust and ESP32 Tools
```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install ESP32 Rust toolchain
cargo install espup
## May need to downgrade to 0.12.2
cargo install espup --version 0.12.2
## then
espup install
## May need to target esp32 specifically
espup update --targets esp32
## then
echo '. /home/josh/export-esp.sh' >> ~/.bashrc
source ~/export-esp.sh

# Install cargo-espflash for uploading
cargo install cargo-espflash
```

### 2. Create Project Structure
```bash
cargo new beep_relay --bin
cd beep_relay
```

### 3. Configure Cargo.toml
Add these dependencies to your `Cargo.toml`:
```toml
[dependencies]
esp-idf-hal = "0.42"
esp-idf-sys = { version = "0.33", features = ["binstart"] }
anyhow = "1.0"

[build-dependencies]
embuild = "0.31"
```

## Upload Process

### 1. Connect Hardware
- Connect ESP32 to computer via USB cable
- Ensure proper driver installation (CP210x or CH340)
- Note the COM port (Windows) or device path (Linux/Mac)

### 2. Build and Flash
```bash
# Build the project
cargo build

# Flash to ESP32 (replace /dev/ttyUSB0 with your port)
cargo espflash flash --target xtensa-esp32-espidf --monitor /dev/ttyUSB0

# Alternative: Build and flash in one step
cargo espflash flash --target xtensa-esp32-espidf --monitor
```

### 3. Monitor Serial Output
```bash
# If not using --monitor flag above
cargo espflash monitor /dev/ttyUSB0
```

## Common Port Names
- **Windows**: `COM3`, `COM4`, etc.
- **Linux**: `/dev/ttyUSB0`, `/dev/ttyACM0`
- **macOS**: `/dev/cu.usbserial-*` or `/dev/cu.SLAB_USBtoUART`

## Troubleshooting Upload Issues

### ESP32 Won't Enter Flash Mode
1. Hold **BOOT** button while pressing **RESET**
2. Release **RESET**, then release **BOOT**
3. Try flashing immediately

### Permission Denied (Linux/Mac)
```bash
sudo usermod -a -G dialout $USER  # Linux
sudo dscl . append /Groups/wheel GroupMembership $(whoami)  # macOS
# Log out and back in
```

### Build Errors
- Ensure `export-esp.sh` is sourced in current terminal
- Check ESP-IDF toolchain installation: `which xtensa-esp32-elf-gcc`
- Clear target directory: `cargo clean`

## Development Workflow
1. Make code changes in `src/main.rs`
2. Build: `cargo build`
3. Flash: `cargo espflash flash --monitor`
4. Observe serial output for debugging
5. Press **RESET** button on ESP32 to restart program

## Useful Commands
```bash
# List available serial ports
cargo espflash board-info

# Flash without monitoring
cargo espflash flash --target xtensa-esp32-espidf

# Monitor only (after flashing)
cargo espflash monitor

# Clean build
cargo clean && cargo build
```