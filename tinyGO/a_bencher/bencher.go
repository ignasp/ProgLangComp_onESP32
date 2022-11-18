package my_bench

import (
	"fmt"
	"time"
	"runtime"
	"machine"
)

var Lang_name = "TinyGo"

type BenchTimer struct {
	tStart    time.Time
	started   bool
	tDuration time.Duration
	cDuration uint32
}

func (self *BenchTimer) Start() {
	self.tStart = time.Now()
	self.started = true
}

func (self *BenchTimer) Stop() {
	self.tDuration = time.Since(self.tStart)
	if self.started != true {
		fmt.Println("Timer was not started duration is invalid")
	}
	self.started = false
	self.cDuration = uint32(float32(self.tDuration.Nanoseconds())/1000 * 160);
}

//-------------------------------------------------------------------------

type RezVerifcation string

const (
	Match   RezVerifcation = "OK"
	NoMatch RezVerifcation = "Err"
	NoStd   RezVerifcation = "NoStd"
)

//-------------------------------------------------------------------------

type PrintType int

const (
	Readable PrintType = iota
	Matlab
	CSV
)

func Print_header(ptype PrintType) {
	switch ptype {
	case Readable:
		fmt.Printf("|%-10s|%-10s|%-10s|%-10s|%-10s|%-10s|%-10s|%-10s\r\n", "Lang", "Test", "Freq(Mhz)", "Iter", "Length", "CPU cyc", "Time us", "Rez")
	case CSV:
		fmt.Printf("%-10s;%-10s;%-10s;%-10s;%-10s;%-10s;%-10s;%-10s\r\n", "Lang", "Test", "Freq(Mhz)", "Iter", "Length", "CPU cyc", "Time us", "Rez")
	case Matlab:
		fmt.Printf("PrintType MATLAB not implemented\r\n")
	}
}

//-------------------------------------------------------------------------

type Tester struct {
	TestName    string
	TestLengths []uint32
	Niterations uint32
	PrintType   PrintType
	RunFn       func(uint32, *BenchTimer) RezVerifcation
}

func (self *Tester) Run() {
	CPU_freq := machine.CPUFrequency() / 1000000
	timer := BenchTimer{}
	for _, dataLen := range self.TestLengths {
		self.RunFn(dataLen, &timer)
		for iters := 0; iters < int(self.Niterations); iters++ {
			rez := self.RunFn(dataLen, &timer)
			switch self.PrintType {
			case Readable:
				fmt.Printf("|%-10s|%-10s|%-10d|%-10d|%-10d|%-10d|%-10d|%-10s\r\n", Lang_name, self.TestName, CPU_freq, iters+1, dataLen, timer.cDuration, timer.tDuration.Microseconds(), rez)
			case CSV:
				fmt.Printf("%-10s;%-10s;%-10d;%-10d;%-10d;%-10d;%-10d;%-10s\r\n", Lang_name, self.TestName, CPU_freq, iters+1, dataLen, timer.cDuration, timer.tDuration.Microseconds(), rez)
			case Matlab:
				fmt.Printf("PrintType MATLAB not implemented\r\n")
			}
			runtime.GC()
		}
	}
}
