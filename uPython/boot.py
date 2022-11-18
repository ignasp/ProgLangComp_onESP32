# This file is executed on every boot (including wake-boot from deepsleep)
#import esp
#esp.osdebug(None)
#import webrepl
#webrepl.start()

import sys
running_on_PC = True; 

if running_on_PC :
    print("Running on PC")
    sys.path.append("./a_pc_scripts")
    sys.path.append("./a_pc_scripts/a_data")
else:
    print("Runing on ESP32")
    sys.path.append("./a_ESP32_scripts")

import main
main.run()