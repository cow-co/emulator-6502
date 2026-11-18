# 6502 Emulator

## Emulating the MOS 6502 Chip, in Particular the 6507 Variant used in Atari 2600

## Design


Like my [8080 emulator](https://github.com/cow-co/8080-emulator), this follows a three-tiered design:

- Emulator layer
  - This emulates the processor itself, and the main memory/device ports
- Machine layer
  - This emulates the machine into which the processor was mounted (eg. an Atari 2600 attached to a TV)
- Host layer
  - This hooks the emulation into the host computer's actual input/output (ie. the actual screen and keyboard which you're using to control it).