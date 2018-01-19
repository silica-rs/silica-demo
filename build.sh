#!/bin/bash

NAME=$(basename $(realpath .))

MODE=
TARGET=cortex-m3
FEATURES="--features olimex-p207"
ARG=""

while [[ $# -gt 0 ]]; do
	key="$1"
	case $key in
	--release)
		MODE="release"
		ARG+="--release"
	;;
	--target)
		TARGET="$2"
		shift
	;;
	--features)
		FEATURES="--features $2"
		case $2 in
		arduino-*)
			TARGET=avr-atmel-none
		;;
		olimex-p207)
			TARGET=cortex-m3
		;;
		*)
			echo "unknown board"
			exit 1
		;;
		esac
		shift
	;;
	*)
		ARG+=$key
	;;
	esac
	shift
done

echo $MODE $TARGET $FEATURES $ARG

if [ -z $MODE ]; then
	MODE=debug
fi

if [ $TARGET == "avr-atmel-none" ]; then
	export XARGO_RUST_SRC=~/Documents/Code/Rust/avr-rust/src
	rustup default avr-atmel-none
else
	rustup default nightly-x86_64-unknown-linux-gnu
fi

#set -x

RUST_TARGET_PATH=. xargo build --target $TARGET $FEATURES $ARG

if [[ $? -eq 0 ]]; then
  arm-none-eabi-objcopy target/$TARGET/$MODE/$NAME -O binary target/$TARGET/$MODE/$NAME.bin
  arm-none-eabi-objcopy target/$TARGET/$MODE/$NAME -O ihex target/$TARGET/$MODE/$NAME.ihex
  arm-none-eabi-objdump -d target/$TARGET/$MODE/$NAME | cargo demangle > target/$TARGET/$MODE/$NAME.disassemble.txt
  arm-none-eabi-nm -lSn target/$TARGET/$MODE/$NAME | cargo demangle > target/$TARGET/$MODE/$NAME.symbolTable.txt
  arm-none-eabi-size --format=SysV target/$TARGET/$MODE/$NAME > target/$TARGET/$MODE/$NAME.memListSummary.txt
fi
#set +x
