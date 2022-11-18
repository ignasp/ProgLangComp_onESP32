
def run():

    from a_bencher import PrintType, Print_header, Tester

    globalPrintType = PrintType.CSV
    globalIterationN = 5

    Print_header(globalPrintType)

    t = Tester()

    from a_tests import t_CRC32_run as runner
    t.TestName =    "CRC32"
    t.TestLengths = [0, 16, 32, 64, 128, 256, 512, 1024]
    t.Niterations = globalIterationN
    t.PrintType =   globalPrintType
    t.RunFn =       runner
    t.Run()
    del runner

    from a_tests import t_SHA256_run as runner
    t.TestName =    "SHA256"
    t.TestLengths = [0, 16, 32, 64, 128, 256, 512, 1024]
    t.Niterations = globalIterationN
    t.PrintType =   globalPrintType
    t.RunFn =       runner
    t.Run()
    del runner

    from a_tests import t_FFT_I16_run as runner
    t.TestName =    "FFT_I16"
    t.TestLengths = [0, 16, 32, 64, 128, 256, 512, 1024]
    t.Niterations = globalIterationN
    t.PrintType =   globalPrintType
    t.RunFn =       runner
    t.Run()
    del runner

    from a_tests import t_iir_biquad_f32_run as runner
    t.TestName =    "IIR_BQ_F32"
    t.TestLengths = [0, 16, 32, 64, 128, 256, 512, 1024]
    t.Niterations = globalIterationN
    t.PrintType =   globalPrintType
    t.RunFn =       runner
    t.Run()
    del runner

    from a_tests import t_fir_f32_run as runner
    t.TestName =    "FIR_F32"
    t.TestLengths = [0, 16, 32, 64, 128, 256, 512, 1024]
    t.Niterations = globalIterationN
    t.PrintType =   globalPrintType
    t.RunFn =       runner
    t.Run()
    del runner
