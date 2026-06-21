# CHIP-8 Opcode Reference (this emulator)

Hand-encoding reference for the opcodes currently implemented in `src/opcodes.rs`.
Each opcode is **2 bytes**, written **big-endian** (high byte first) into the `.ch8` file.

## Encoding placeholders

When you see a letter in an opcode pattern, substitute a hex digit:

| Symbol | Width   | Meaning                                                        |
|--------|---------|---------------------------------------------------------------|
| `X`    | 1 nibble| First register number, `0`–`F` (selects `V0`–`VF`)            |
| `Y`    | 1 nibble| Second register number, `0`–`F`                               |
| `N`    | 1 nibble| 4-bit constant (`0`–`F`)                                       |
| `NN`   | 1 byte  | 8-bit constant (`0x00`–`0xFF`)                                 |
| `NNN`  | 12 bits | Address (`0x000`–`0xFFF`); your programs live at `0x200`+     |

Example: opcode `7XNN` with X=`A`, NN=`15` → `0x7A15` → bytes `7A 15`.

## Registers

| Register   | Notes                                                          |
|------------|----------------------------------------------------------------|
| `V0`–`VE`  | 15 general-purpose 8-bit registers                            |
| `VF`       | Flag register — **do not use as a scratch register**; opcodes overwrite it with carry/borrow/shift results |
| `I`        | 16-bit index/address register (set by `ANNN`)                 |
| `PC`       | Program counter; execution starts at `0x200`                  |
| stack/`SP` | Return-address stack for `CALL`/`RET` (16 deep)               |

## Implemented opcodes

| Opcode  | Mnemonic        | Effect                                                                 |
|---------|-----------------|------------------------------------------------------------------------|
| `00E0`  | CLS             | Clear screen — **stub, does nothing yet**                              |
| `00EE`  | RET             | Return from subroutine (pop return address off stack into PC)          |
| `1NNN`  | JP   NNN        | Jump: `PC = NNN`                                                        |
| `2NNN`  | CALL NNN        | Call subroutine at `NNN` (pushes return address)                       |
| `3XNN`  | SE   VX, NN     | Skip next instruction if `VX == NN`                                    |
| `4XNN`  | SNE  VX, NN     | Skip next instruction if `VX != NN`                                    |
| `5XY0`  | SE   VX, VY     | Skip next instruction if `VX == VY` (last nibble must be `0`)          |
| `6XNN`  | LD   VX, NN     | `VX = NN`                                                              |
| `7XNN`  | ADD  VX, NN     | `VX = VX + NN` (wraps on overflow; **does not** set VF)                |
| `8XY0`  | LD   VX, VY     | `VX = VY`                                                              |
| `8XY1`  | OR   VX, VY     | `VX = VX | VY`                                                         |
| `8XY2`  | AND  VX, VY     | `VX = VX & VY`                                                         |
| `8XY3`  | XOR  VX, VY     | `VX = VX ^ VY`                                                         |
| `8XY4`  | ADD  VX, VY     | `VX = VX + VY`; `VF = 1` if it overflowed past 0xFF, else `0`          |
| `8XY5`  | SUB  VX, VY     | `VX = VX - VY`; `VF = 1` if **no** borrow (`VX >= VY`), else `0`       |
| `8XY6`  | SHR  VX         | `VF = (VX & 1)` (lost low bit); then `VX = VX >> 1` (divide by 2)      |
| `8XY7`  | SUBN VX, VY     | `VX = VY - VX`; `VF = 1` if **no** borrow (`VY >= VX`), else `0`       |
| `8XYE`  | SHL  VX         | `VF = (VX >> 7)` (lost high bit); then `VX = VX << 1` (multiply by 2)  |
| `9XY0`  | SNE  VX, VY     | Skip next instruction if `VX != VY` (last nibble must be `0`)          |
| `ANNN`  | LD   I, NNN     | `I = NNN`                                                              |

> Note on the shift ops (`8XY6` / `8XYE`): this emulator shifts `VX` in place and
> ignores `Y`. (Some other CHIP-8 variants shift `VY` into `VX` — yours does not.)

> Note on `VF`: for `8XY4/5/6/E`, the flag is written **after** the result. If you
> ever use `VF` as the destination `X`, the arithmetic result is immediately
> clobbered by the flag. Keep results in `V0`–`VE`.

## Worked example — a few "random calculations"

A program is just these opcodes laid end to end starting at `0x200`. Bytes go in
the file in the order listed, high byte then low byte:

| Address | Opcode | Bytes   | Meaning                          | Result        |
|---------|--------|---------|----------------------------------|---------------|
| 0x200   | `6005` | `60 05` | `V0 = 5`                         | V0 = 0x05     |
| 0x202   | `610A` | `61 0A` | `V1 = 10`                        | V1 = 0x0A     |
| 0x204   | `8014` | `80 14` | `V0 = V0 + V1`                   | V0 = 0x0F     |
| 0x206   | `7003` | `70 03` | `V0 = V0 + 3`                    | V0 = 0x12     |
| 0x208   | `8204` | `82 04` | `V2 = V2 + V0` (V2 started 0)    | V2 = 0x12     |
| 0x20A   | `8021` | `80 21` | `V0 = V0 | V1`                   | V0 = 0x1F     |

Raw file bytes: `60 05 61 0A 80 14 70 03 82 04 80 21`

Run that many cycles (6 here), then `dump_registers()` should show
`V0 = 0x1F`, `V1 = 0x0A`, `V2 = 0x12`.
