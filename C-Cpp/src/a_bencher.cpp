#include "a_data.h"
#include "a_bencher.h"

void benchTimer_start(BenchTimer * self){
    self->tStart = esp_timer_get_time();
    self->cStart = xthal_get_ccount();
	self->started = true;
}

void benchTimer_stop(BenchTimer * self){
    self->tDuration = esp_timer_get_time() - self->tStart;
    self->cDuration = xthal_get_ccount() - self->cStart;
	if (self->started != true) {
		printf("Timer was not started duration is invalid");
	}
	self->started = false;
}

const char * printVerification(RezVerifcation ver){
    switch(ver){
        case RezVerifcation::Match: return "OK";
        case RezVerifcation::NoMatch: return "Err";
        case RezVerifcation::NoSTD: return "NoSTD";
        case RezVerifcation::NotRun: return "Not run";
        default: return "Bad Verificatio code";
    }
}

void bench_Print_header(PrintType ptype){
    switch(ptype){
        case PrintType::Readable: 
            printf("|%-10s|%-10s|%-10s|%-10s|%-10s|%-10s|%-10s|%-10s\r\n", "Lang", "Test", "Freq(Mhz)", "Iter", "Length", "CPU cyc", "Time us", "Rez");
            break;
        case PrintType::CSV:
            printf("%-10s;%-10s;%-10s;%-10s;%-10s;%-10s;%-10s;%-10s\r\n", "Lang", "Test", "Freq(Mhz)", "Iter", "Length", "CPU cyc", "Time us", "Rez");
            break;
        case PrintType::Matlab:
            printf("PrintType MATLAB not implemented\r\n");
            break;
    }
}

void bench_Run(Tester * self){
    uint32_t Freq = getCpuFrequencyMhz();
    BenchTimer timer;
    timer.cDuration = 0;
    timer.tDuration = 0;
    int i = 0; 
    while (self->TestLengths[i] != -1){
        int32_t dataLen = self->TestLengths[i++];
        int iters = 0;
        RezVerifcation rez = RezVerifcation::NotRun;
        unsigned intlevel = portENTER_CRITICAL_NESTED();
        rez = self->RunFn(dataLen, &timer);
        for (; iters < self->Niterations; iters++){
            rez = self->RunFn(dataLen, &timer);
            switch(self->Ptype) {
                case Readable:
                    printf("|%-10s|%-10s|%-10d|%-10d|%-10d|%-10d|%-10d|%-10s\r\n", Lang_name, self->TestName, Freq, iters+1, dataLen, timer.cDuration, timer.tDuration, printVerification(rez));
                    break;
                case CSV:
                    printf("%-10s;%-10s;%-10d;%-10d;%-10d;%-10d;%-10d;%-10s\r\n", Lang_name, self->TestName, Freq, iters+1, dataLen, timer.cDuration, timer.tDuration, printVerification(rez));
                    break;
                case Matlab:
                    printf("PrintType MATLAB not implemented\r\n");
                    break;
            }
        }
    }
}