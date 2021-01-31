
#! /usr/bin/zsh

set -e

if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "usage: $0 <path-to-binary.elf>" >&2
    exit 1
fi

if [ "$#" -lt 1 ]; then
    echo "$0: Expecting a .elf file" >&2
    exit 1
fi

/home/kevlar/Documents/arduino-1.8.13/hardware/tools/avr/bin/avrdude -C/home/kevlar/Documents/arduino-1.8.13/hardware/tools/avr/etc/avrdude.conf -v -patmega328p -carduino -P/dev/ttyUSB0 -b57600  -D "-Uflash:w:${1}:e"

