#ifndef A_DATA_h
#define A_DATA_h

#include <Arduino.h>

extern const char * Lang_name;
//extern uint32_t Freq;

#define DATA_LEN 4096
extern const uint8_t data[ 4096 ];

int32_t data_Find_std(const int32_t * len_vec, int32_t len);

extern const  int32_t  CRC32_rez_lens[];
extern const  uint32_t CRC32_rez[][1];

extern const  int32_t SHA256_rez_lens[];
extern const  uint8_t SHA256_rez[][32];

extern const  int32_t FFT_I16_rez_lens[];
extern const  int16_t FFT_I16_rez[][4096];

extern const  int32_t IIR_BQ_rez_lens[];
extern const  float IIR_BQ_rez[][4096];

extern const int32_t FIR_rez_lens[];
extern const float FIR_rez[][4096];

#endif