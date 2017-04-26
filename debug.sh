#!/bin/bash
arm-none-eabi-gdb target/cortex-m3/debug/silica-demo -iex 'target remote localhost:3333'
