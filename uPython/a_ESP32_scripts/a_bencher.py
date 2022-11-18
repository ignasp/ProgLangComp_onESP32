import gc

from utime import ticks_us, ticks_diff, ticks_cpu
import micropython, machine

Lang_name = "uPython"

class BenchTimer:
    def __init__(self):
        self.tStart = 0
        self.cStart = 0
        self.started = False
        self.tDuration = 0
        self.cDuration = 0
        
    def start(self):
        self.tStart = ticks_us()
        self.cStart = ticks_cpu()
        self.started = True
        
    def stop(self):
        self.tDuration = ticks_diff(ticks_us(), self.tStart)
        self.cDuration = ticks_cpu() - self.cStart 
        if self.started != True :
            print("Timer was not started duration is invalid")
        self.started = False
        
##-------------------------------------------------------------------------
class PrintType:
    Readable = 0
    Matlab = 1
    CSV = 2
    
def Print_header(ptype):
    if ptype == PrintType.Readable:
        print("|%10s|%10s|%10s|%10s|%10s|%10s|%10s|%10s" % ("Lang", "Test", "Freq(Mhz)", "Iter", "Length", "CPU cyc", "Time us", "Rez"))
    elif ptype == PrintType.CSV:
        print("%10s;%10s;%10s;%10s;%10s;%10s;%10s;%10s" % ("Lang", "Test", "Freq(Mhz)", "Iter", "Length", "CPU cyc", "Time us", "Rez"))
    elif ptype == PrintType.Matlab:
        print("PrintType MATLAB not implemented\r\n")
    else:
        print("No such printtype\r\n")

##-------------------------------------------------------------------------
class RezVerifcation:
    Match    = "OK"
    NoMatch  = "Err"
    NoStd    = "NoStd"
    
##-------------------------------------------------------------------------
    
class Tester:
    TestName    = ""
    TestLengths = []
    Niterations = 0
    PrintType   = PrintType.Readable
    RunFn       = []
    
    def Run(self):
        Freq = int(machine.freq()/1000000)
        timer = BenchTimer()
        for dataLen in self.TestLengths:
            for iters in range(self.Niterations):
                rez = self.RunFn(dataLen, timer)
                if self.PrintType == PrintType.Readable:
                    print("|%10s|%10s|%10s|%10s|%10s|%10s|%10s|%10s" % (Lang_name, self.TestName, Freq, iters+1, dataLen, timer.cDuration, timer.tDuration, rez))
                elif self.PrintType == PrintType.CSV:
                    print("%10s;%10s;%10s;%10s;%10s;%10s;%10s;%10s" % (Lang_name, self.TestName, Freq, iters+1, dataLen, timer.cDuration, timer.tDuration, rez))
                elif self.PrintType == PrintType.Matlab:
                    print("PrintType MATLAB not implemented\r\n")
                else:
                    print("No such printtype\r\n")
            
        

        
    