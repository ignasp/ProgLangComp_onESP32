#![allow(nonstandard_style)]
#![allow(dead_code)]

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

mod a_data;
mod a_bencher;

mod t_fft_2r_sc16_ansi;
mod t_sha256;
mod t_crc32_ieee;
mod t_iir_biquad_f32_ansi;
mod t_fir_f32_ansi;

mod a_tests;

use crate::a_bencher::*;
use crate::a_tests::tests;

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    const globalPrintType : my_bench::printType = my_bench::printType::CSV;
    const globalIterationN : u32 = 100;

    my_bench::print_header(globalPrintType);

    let _t1 = my_bench::tester{
        testName    : "CRC32",
        testLengths : vec![0, 16, 32, 64, 128, 256, 512, 1024],
        nIterations : globalIterationN,
        printType   : globalPrintType,
        runFn       : tests::crc32_run,
    }.run();

    let _t2 = my_bench::tester{
        testName    : "SHA256",
        testLengths : vec![0, 16, 32, 64, 128, 256, 512, 1024],
        nIterations : globalIterationN,
        printType   : globalPrintType,
        runFn       : tests::sha256_run,
    }.run();

    let _t3 = my_bench::tester{
        testName    : "FFT_I16",
        testLengths : vec![0, 16, 32, 64, 128, 256, 512, 1024],
        nIterations : globalIterationN,
        printType   : globalPrintType,
        runFn       : tests::fft_2r_sc16_ansi_run,
    }.run();

    let _t4 = my_bench::tester{
        testName    : "IIR_BQ_F32",
        testLengths : vec![0, 16, 32, 64, 128, 256, 512, 1024],
        nIterations : globalIterationN,
        printType   : globalPrintType,
        runFn       : tests::iir_biquad_f32_ansi_run,
    }.run();

    let _t5 = my_bench::tester{
        testName    : "FIR_F32",
        testLengths : vec![0, 16, 32, 64, 128, 256, 512, 1024],
        nIterations : globalIterationN,
        printType   : globalPrintType,
        runFn       : tests::fir_f32_ansi_run,
    }.run();
    
}
