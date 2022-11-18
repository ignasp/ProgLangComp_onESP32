#ifndef A_TESTS_h
#define A_TESTS_h

#include <Arduino.h>
#include "a_data.h"
#include "a_bencher.h"


RezVerifcation t_crc32_run(int32_t data_len, BenchTimer * timer);
RezVerifcation t_sha256_run(int32_t data_len, BenchTimer * timer);
RezVerifcation t_ffti16_run(int32_t data_len, BenchTimer * timer);
RezVerifcation t_iir_biquad_f32_run(int32_t data_len, BenchTimer * timer);
RezVerifcation t_fir_f32_run(int32_t data_len, BenchTimer * timer);

#endif