# Protocol Description

Messages sent to the controller will be sent over RS232 at 115,200 baud.
Messages will be encoded using the [COBS algorithm](https://en.wikipedia.org/wiki/Consistent_Overhead_Byte_Stuffing)
and will always be immediately followed by a `0x00` byte to signal their
completion.

The data contained within the message will be structured as:

| **Byte**     | 0                | ...                    | n-1 | n   |
|--------------|------------------|------------------------|-----|-----|
| **Contents** | Instruction code | [Instruction contents] | CRC | CRC |

Due to the COBS encoding being used, the length of the message is limited
to 253 bytes (plus 2 bytes for CRC16 + 1 byte for COBS overhead).

## CRC

Bytes 0 through `n-2` will have a checksum computed using CRC16 with the
following parameters applied:

```
ARC

Width:          16
Polynomial:     0x8005
Initial:        0x0000
Reverse input:  true
Reverse output: true
XOR out:        0x0000
Check:          0xbb3d
Residue:        0x0000
```

The CRC is written into the message in Big Endian order.

## Instructions

Each instruction is transmitted as a byte defining the instruction followed by
the data specific to that instruction.

### SetLeds

This instruction is used to set the colors for one or more LEDs in the
controlled strip and is denoted by the instruction code `0x01`.

| **Byte**     | 0    | 1      | 2          | 3         | 4           | 5          | ... | n-1 | n   |
|--------------|------|--------|------------|-----------|-------------|------------|-----|-----|-----|
| **Contents** | 0x01 | offset | num_pixels | led0[red] | led0[green] | led0[blue] | ... | CRC | CRC |

After the instruction code, the offset (ie, the index of the LED in the strip,
index base 0) of the initial LED to set is specified using a single unsigned
byte.  The next byte specifies the number of pixels being set by this message
as an unsigned integer, with a minimum value of 1 and a maximum value of 83.
The limitations of the offset and pixel count fields result in a maximum LED
count of `255 + 83 = 338`.

Following the metadata fields in the instruction, the actual LED colors are
specified in sets of three bytes for red, blue, and green values, respectively.
Each value is specified using an unsigned one byte integer, allowing values
from 0 to 255, like HTML RGB colors.

Setting `num_pixels` to a value less than the actual number of pixels defined
in the instruction will result in the pixels beyond the specified number being
ignored.  Setting `num_pixels` higher than the actual number of pixels defined
by the message will result in an out of bounds error.
