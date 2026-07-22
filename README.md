# raytracer-rs
Raytracer in Rust for educational purposes

## Prerequisites
You need to have `cargo` installed. This project requires Rust version 1.85+.
On debian-based systems you can check the version with:
```bash
apt-cache policy rustc
```

If you see version >= 1.85, you can install it from `apt` with:
```bash
sudo apt install -y build-essential cargo rustc
```

If you see older version, you can install the latest version of Rust using rustup:
```bash
sudo apt install -y build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

## Build and run
### With preview window
To build the project execute
```bash
./build.sh
```

To run the project execute
```bash
./run.sh
```

### Without preview window (headless systems)
For a headless build without the GUI preview window, use:
```bash
./build-headless.sh
```

To run the headless build, use:
```bash
./run-headless.sh
```