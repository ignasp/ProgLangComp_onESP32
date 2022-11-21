# MicroPython project for performance evaluation on ESP32
## Prerequisites
MicroPython for ESP32 can be installed by following [these instructions](https://docs.micropython.org/en/latest/esp32/tutorial/intro.html).
For M5 Stack Basic, [generic ESP32 firmware](https://docs.micropython.org/en/latest/esp32/tutorial/intro.html) can be used.
For editing, running or uploading the code on ESP32, an IDE like [Thonny](https://thonny.org/) can be used. 

## Running the code
Generrely, the code can be run on any, Python3 interpretes. However, since some ESP32 specific libraries are only available in ESP32 MicroPython firmware, two versions of some files had to be written. Also, ESP32 code version contains limited verification data (in the `a_data.py` file), since the full file cannot be loaded due to insufficient memory. 
- `a_ESP32_scripts` folder contains ESP32 specific code.
- `a_pc_scripts` folder contains generic Python3 code.

the code version is determined by the 'running_on_PC` variable in `boot.py` file. If the code is intended to be run on ESP32, it should be set to `False`.

To run the code on ESP32, all files in this folder, except for `a_pc_scripts` should be uploaded to ESP32. The then should be run on boot.
