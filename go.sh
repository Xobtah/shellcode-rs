#!/bin/zsh
cargo +nightly build --target x86_64-pc-windows-msvc --release && \
  objcopy -j .text -O binary target/x86_64-pc-windows-msvc/release/shellcode.exe sc.bin && \
  cat sc.bin | xxd -i > sc.x && \
  x86_64-w64-mingw32-gcc runner.c -o sc.exe && \
  mv sc.exe ~/Desktop