# chip8-rust

![Alt text](https://github.com/PierreLeidbring/chip8-rust/blob/master/img/c8.png "Screenshot")

## Intent
This project is intended as a program suite focused on the chip8.  
Upon completion it is supposed to contain:  
* An emulator/vm for the chip8
* A debugger
* A disassembler
* An assembler

## Background
The entire project is written in Rust witht the addition of sfml-bindings for rust.  
Worth noting is that this is written solely for my own amusement and with the sole purpose of me learning more about emulators and vms.
Whoever want to try it out or even modify it are welcome to do so!  
Please feel free to make pull requests if you have ways to better the code or find bugs.  

## Current state
~~Right now the emulator is working albeit with a couple of bugs when playing certain roms.  
The Trip8 demo, The zero demo and BRIX all work pretty much as intended.~~
All roms I've tested now works properly. (NOT super chip8 or HP48)

## Issues and wanted features
### Emulator
- [x] Clockspeed/timer countdown
- [x] Freezing upon collision
- [x] (Invaders) Restart upon killing an entire wave
- [x] (Brix) Fail to restart when out of lives

#### Main menu
- [ ] Choose foreground color
- [ ] Choose background color
- [ ] Save config
- [ ] Choose rom via file browser

### Debugger
Currently I am working on finding the bug that causes the strange behaviours mentioned in the section above (Space invaders and Brix). Once they are gone I will focus on getting a working debug mode in the emulator. This will allow for the emulator to work both as an emulator and as a debugger for roms you write yourself.
### Disassembler
* TBA
### Assembler
* TBA
### Disclaimer
Even though the "supermode" flag exists the emulator is not fully compatible with super chip 8 games. The only thing this actually change is a small detail in the 8XY6 instruction.

## Instructions
### Dependencies
* csfml
* sfml

Clone and compile using cargo.
