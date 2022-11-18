package main

import (
	bench "tinyGO/a_bencher"
	tests "tinyGO/a_tests"
)

func main() {

	globalPrintType := bench.CSV
	globalIterationN := uint32(100)

	bench.Print_header(globalPrintType)

	t1 := bench.Tester{
		TestName:    "CRC32",
		TestLengths: []uint32{0, 16, 32, 64, 128, 256, 512, 1024},
		Niterations: globalIterationN,
		PrintType:   globalPrintType,
		RunFn:       tests.CRC32_run,
	}
	t1.Run()

	t2 := bench.Tester{
		TestName:    "SHA256",
		TestLengths: []uint32{0, 16, 32, 64, 128, 256, 512, 1024},
		Niterations: globalIterationN,
		PrintType:   globalPrintType,
		RunFn:       tests.SHA256_run,
	}
	t2.Run()

	
	t3 := bench.Tester{
		TestName:    "FFT_I16",
		TestLengths: []uint32{0, 16, 32, 64, 128, 256, 512, 1024},
		Niterations: globalIterationN,
		PrintType:   globalPrintType,
		RunFn:       tests.FFT_I16_run,
	}
	t3.Run()
	

	t4 := bench.Tester{
		TestName:    "FIR_F32",
		TestLengths: []uint32{0, 16, 32, 64, 128, 256, 512, 1024},
		Niterations: globalIterationN,
		PrintType:   globalPrintType,
		RunFn:       tests.FIR_F32_run,
	}
	t4.Run()
	
	
	t5 := bench.Tester{
		TestName:    "IIR_BQ_F32",
		TestLengths: []uint32{0, 16, 32, 64, 128, 256, 512, 1024},
		Niterations: globalIterationN,
		PrintType:   globalPrintType,
		RunFn:       tests.IIR_F32_run,
	}
	t5.Run()

	for {
	}
}
