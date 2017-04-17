#!/bin/bash
[ -z $1 ] && { MODE="debug"; } || { MODE=$1; }
[ -z $2 ] && { TARGET="cortex-m3"; } || { TARGET=$2; }
[ $MODE == "release" ] && { ARG="--$MODE"; }

export RUST_TARGET_PATH=`pwd`
xargo build $ARG --target $TARGET

if [[ $? -eq 0 ]]; then
  arm-none-eabi-objcopy target/$TARGET/$MODE/silica-olimex-p207-demo -O binary target/$TARGET/$MODE/silica-olimex-p207-demo.bin
  arm-none-eabi-objcopy target/$TARGET/$MODE/silica-olimex-p207-demo -O ihex target/$TARGET/$MODE/silica-olimex-p207-demo.ihex
  arm-none-eabi-objdump -d target/$TARGET/$MODE/silica-olimex-p207-demo | cargo demangle > target/$TARGET/$MODE/silica-olimex-p207-demo.disassemble.txt
  arm-none-eabi-nm -lSn target/$TARGET/$MODE/silica-olimex-p207-demo | cargo demangle > target/$TARGET/$MODE/silica-olimex-p207-demo.symbolTable.txt
  arm-none-eabi-size --format=SysV target/$TARGET/$MODE/silica-olimex-p207-demo > target/$TARGET/$MODE/silica-olimex-p207-demo.memListSummary.txt
fi
