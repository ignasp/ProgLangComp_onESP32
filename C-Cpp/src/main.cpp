#include <Arduino.h>
#include <malloc.h>

#include "a_data.h"
#include "a_bencher.h"
#include "a_tests.h"

//-----------------------------------------------------------------------

PrintType globalPrintType = PrintType::CSV;
uint32_t globalIterationN = 100;

void setup() {
  // put your setup code here, to run once:
  Serial.begin(115200);
  setCpuFrequencyMhz(160);
  delayMicroseconds(1000*1000);

  bench_Print_header(globalPrintType);
}

void loop() {

	int32_t TestLengths1[] = {0, 16, 32, 64, 128, 256, 512, 1024, -1};
	Tester t1 = {
		TestName : 	 "CRC32",
		TestLengths: TestLengths1,
		Niterations: globalIterationN,
		Ptype:   	 globalPrintType,
		RunFn:       &t_crc32_run
	};
	bench_Run(&t1);

	int32_t TestLengths2[] = {0, 16, 32, 64, 128, 256, 512, 1024, -1};
	Tester t2 = {
		TestName : 	 "SHA256",
		TestLengths: TestLengths2,
		Niterations: globalIterationN,
		Ptype:   	 globalPrintType,
		RunFn:       &t_sha256_run
	};
	bench_Run(&t2);

	int32_t TestLengths3[] = {0, 16, 32, 64, 128, 256, 512, 1024, -1};
	Tester t3 = {
		TestName : 	 "FFT_I16",
		TestLengths: TestLengths3,
		Niterations: globalIterationN,
		Ptype:   	 globalPrintType,
		RunFn:       &t_ffti16_run
	};
	bench_Run(&t3);

	int32_t TestLengths4[] = {0, 16, 32, 64, 128, 256, 512, 1024, -1};
	Tester t4 = {
		TestName : 	 "IIR_BQ_F32",
		TestLengths: TestLengths4,
		Niterations: globalIterationN,
		Ptype:   	 globalPrintType,
		RunFn:       &t_iir_biquad_f32_run
	};
	bench_Run(&t4);

	int32_t TestLengths5[] = {0, 16, 32, 64, 128, 256, 512, 1024, -1};
	Tester t5 = {
		TestName : 	 "FIR_F32",
		TestLengths: TestLengths5,
		Niterations: globalIterationN,
		Ptype:   	 globalPrintType,
		RunFn:       &t_fir_f32_run
	};
	bench_Run(&t5);

	while(1){}

   //delayMicroseconds(30*1000*1000);
}