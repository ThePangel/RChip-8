# RChip-8

A simple CHIP-8 interpreter written in Rust as a learning project to explore emulator development.

---

## Overview

RChip-8 is a straightforward implementation of the CHIP-8 virtual machine built from scratch in Rust. The primary goal of this project is to better understand how emulators work, including instruction decoding, memory management, timers, input handling, graphics, and sound.

### Dependencies

The project uses a minimal set of libraries:

* `winit` – Cross-platform window creation and event handling.
* `pixels` – Pixel-based rendering of the 64×32 CHIP-8 display.
* `rand` – Implements the `Cxkk` (random number) opcode.
* `rodio` – Generates the CHIP-8 beep sound.

---

## Running

### From source

Clone the repository:

```bash
git clone https://github.com/ThePangel/RChip-8.git
cd RChip-8
```

Run the emulator with a ROM:

```bash
cargo run -- path/to/rom
```

### Prebuilt binaries

Precompiled binaries are available on the GitHub Releases page.

Run the executable and pass the ROM path as the first command-line argument:

```bash
rchip-8 path/to/rom
```




---

