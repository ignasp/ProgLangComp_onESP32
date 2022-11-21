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

'MATLAB models' folder contains MATLAB models for each tested function. These models are used to generate reference output data.

All projects were run on an [M5stack Core Basic](https://docs.m5stack.com/en/core/basic), but any ESP32 based board should work.

## Project structure
The project for each language is in a separate directory. The structure of these projects is kept as similar as possible. 
- `a_bencher` file contains a simple test bench structure, timer structure and test runner function. For this testbench a wrapper funcion must be defined - it runs the tested function and also includes the necessary initialization and cleanup. 
- `a_tests` file contains currently used wrapper functions. 
- `t_[funcion_name]` files conatin the source code for actual tested functions.
- `a_data` contains test input data (an array of random uint8 type values) and expected values for each function.
- `main` file is where the tesbench structures are initialized and executed. 

## Usage
After flashing the code, it is run immediately after boot. The test results are printed to the serial console. For viewing the output, a terminal emulator like [MobaXterm](https://mobaxterm.mobatek.net/) can be used, with baudrate setting 115200. The output is in CSV. A more readable format can be set by changing the variable `globalPrintType` to `PrintType::Readable` in `main` file.

Instructions on how to compile and flash each project are in their respective directories.
