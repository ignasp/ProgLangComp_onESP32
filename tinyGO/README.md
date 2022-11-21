# TinyGo project for performance evaluation on ESP32
## Prerequisites
TinyGo for ESP32 can be installed by following [these guides](https://tinygo.org/getting-started/install/).

## Building and flashing
While in this directory, use 
- `tinygo build -target=m5stack main.go` to just build the code, or
- `tinygo flash -target=m5stack -port=[ESP32 serial port] tinyGO` to build and flash it.

Currently, TInyGO fails to link the code with full input and expected output data -`a_data` file. For now, the `a_data2` is used, which contains 32 points of expected data.
