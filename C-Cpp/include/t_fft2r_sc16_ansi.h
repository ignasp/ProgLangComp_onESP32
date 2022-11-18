#ifndef FFT2R_SC16_ANSI_h
#define FFT2R_SC16_ANSI_h

#include "a_data.h"

typedef struct sc16_u
{
    int16_t re;
    int16_t im;
}sc16_t;

extern  int16_t FFT_TABLE_I16[4096];

bool dsps_fft2r_sc16_ansi_(int16_t *data, int N, int16_t* sc_table);

#endif