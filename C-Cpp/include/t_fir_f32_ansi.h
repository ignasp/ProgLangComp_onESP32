#ifndef T_FIR_F32_ANSI_h
#define T_FIR_F32_ANSI_h

/**
 * @brief Data struct of f32 fir filter
 *
 * This structure used by filter internally. User should access this structure only in case of
 * extensions for the DSP Library.
 * All fields of this structure initialized by dsps_fir_init_f32(...) function.
 */
typedef struct fir_f32_s {
    float  *coeffs;     /*!< Pointer to the coefficient buffer.*/
    float  *delay;      /*!< Pointer to the delay line buffer.*/
    int     N;          /*!< FIR filter coefficients amount.*/
    int     pos;        /*!< Position in delay line.*/
    int     decim;      /*!< Decimation factor.*/
    int     d_pos;      /*!< Actual decimation counter.*/
} fir_f32_t;


bool dsps_fir_init_f32(fir_f32_t *fir, float *coeffs, float *delay, int N);
bool dsps_fir_f32_ansi(fir_f32_t *fir, const float *input, float *output, int len);

#endif