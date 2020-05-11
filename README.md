# chip8-rs

A CHIP-8 Emulator written in Rust.

## Keypad

The original CHIP-8 had a 16-key hexadecimal keypad with the following layout:

|   |   |   |   |
|---|---|---|---|
| 1 | 2 | 3 | C |
| 4 | 5 | 6 | D |
| 7 | 8 | 9 | E |
| A | 0 | B | F |

The following is the implemented layout that better fits the QWERTY keyboard layout:

|   |   |   |   |
|---|---|---|---|
| Q | W | E | R |
| A | S | D | F |
| U | I | O | P |
| J | K | L | ; |


