# Performance Evaluation of C/C++, MicroPython, Rust and TinyGo on ESP32 
This project compares execution time of several well known algorithms written in different languages on [ESP32](https://espressif.com/en/products/hardware/esp32/overview) based [M5stack Core Basic](https://docs.m5stack.com/en/core/basic)

## Overview
The languages compared are:
- [C/C++](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/index.html)
- [Rust](https://esp-rs.github.io/book/)
- [TinyGO](https://tinygo.org/docs/reference/microcontrollers/m5stack/)
- [MicroPython](https://docs.micropython.org/en/latest/esp32/tutorial/intro.html)

Five functions were executed in each language. The functions (algortihms) are taken from open sources and slightly modified, and ported to other languages, trying to keep the code a similar as possible between languages (links to original sources):
- [CRC32](https://cs.opensource.google/go/go/+/refs/tags/go1.19.3:src/hash/crc32/crc32_generic.go)
- [SHA256](https://github.com/B-Con/crypto-algorithms/blob/master/sha256_test.c)
- [FFT(fixed point, complex pair of int16)](https://github.com/espressif/esp-dsp/blob/master/modules/fft/fixed/dsps_fft2r_sc16_ansi.c)
- [IIR(floating point)](https://github.com/espressif/esp-dsp/blob/master/modules/iir/biquad/dsps_biquad_f32_ansi.c)
- [FIR(floating point)](https://github.com/espressif/esp-dsp/blob/master/modules/fir/float/dsps_fir_f32_ansi.c)

