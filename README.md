# CHIP-8 Emulator

A simple CHIP-8 emulator written in Rust.

Features:

- ✅ Full CHIP-8 instruction set
- ✅ Sound support
- ✅ Keyboard input
- ✅ Configurable ROM loading
- ✅ 64×32 monochrome display
- ✅ Passes the Timendus CHIP-8 test suite

---

## Controls

The emulator uses the standard CHIP-8 keyboard layout.

| CHIP-8 | Keyboard |
|--------:|:--------|
| 1 | 1 |
| 2 | 2 |
| 3 | 3 |
| C | 4 |
| 4 | Q |
| 5 | W |
| 6 | E |
| D | R |
| 7 | A |
| 8 | S |
| 9 | D |
| E | F |
| A | Z |
| 0 | X |
| B | C |
| F | V |

---

## Running

Download the executable for your platform.

Then launch a ROM by passing its path as the first argument:

```bash
./chip8-emulator <rom>
```

Example:

```bash
./chip8-emulator roms/Tetris.ch8
```

On Windows:

```powershell
chip8-emulator.exe roms\Tetris.ch8
```

---

## Building from Source

Clone the repository:

```bash
git clone https://github.com/<your-username>/chip8-emulator.git
cd chip8-emulator
```

Build in release mode:

```bash
cargo build --release
```

The executable will be located in:

```
target/release/chip8-emulator
```

Run it:

```bash
cargo run --release -- roms/Pong.ch8
```

---

## Tested ROMs

- IBM Logo
- Pong
- Tetris
- Airplane
- Timendus CHIP-8 Test Suite
- Corax89 Test ROM

---

## Dependencies

- Rust
- minifb
- rodio
- rand

---

## License

MIT