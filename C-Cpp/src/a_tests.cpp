#include "a_tests.h"
#include "a_data.h"
#include "a_bencher.h"

#include "t_crc32_ieee.h"
#include "t_sha256.h"
#include "t_fft2r_sc16_ansi.h"
#include "t_iir_biquad_f32_ansi.h"
#include "t_fir_f32_ansi.h"

RezVerifcation t_crc32_run(int32_t data_len, BenchTimer * timer){
    // Init
    uint32_t table[256];
    crc32_make_table(0xEDB88320, table);
    uint8_t * in_data = (uint8_t *)calloc(data_len, sizeof(uint8_t));
    for (int i = 0; i < data_len; i++){
        in_data[i] = data[i];
    }

    // Run
    benchTimer_start(timer);
    uint32_t rez = crc32_calc(in_data, data_len, table);
    benchTimer_stop(timer);

    // Verify
    RezVerifcation ver = RezVerifcation::NoSTD;
    int32_t rez_ind = data_Find_std(CRC32_rez_lens, data_len);
    if (rez_ind >= 0){
       ver = RezVerifcation::NoMatch;
       if (rez == CRC32_rez[rez_ind][0]){
            ver = RezVerifcation::Match;
        }
    }
    delete(in_data);
    return ver;
}

RezVerifcation t_sha256_run(int32_t data_len, BenchTimer * timer){
    // Init
    SHA256_CTX ctx_v;
    BYTE rez[SHA256_BLOCK_SIZE];
    uint8_t * in_data = (uint8_t *)calloc(data_len, sizeof(uint8_t));
    for (int i = 0; i < data_len; i++){
        in_data[i] = data[i];
    }

    // Run
    benchTimer_start(timer);

    sha256_initialize(&ctx_v);
    sha256_update(&ctx_v, in_data, data_len);
	sha256_final(&ctx_v, rez);

    benchTimer_stop(timer);

    // Verify
     RezVerifcation ver = RezVerifcation::NoSTD;
    int32_t rez_ind = data_Find_std(CRC32_rez_lens, data_len);
    if (rez_ind >= 0){
        ver = RezVerifcation::Match;
        for(int i = 0; i < SHA256_BLOCK_SIZE; i++){
            if (rez[i] != SHA256_rez[rez_ind][i]){
                ver = RezVerifcation::NoMatch;
            }
        }
    }
    delete(in_data);
    return ver;
}

RezVerifcation t_ffti16_run(int32_t data_len, BenchTimer * timer){
     // Init
    int16_t * inout_data = (int16_t *)calloc(2*data_len, sizeof(int16_t));
	for(int i = 0; i < data_len*2; i++)
	{
		inout_data[i] = data[i];
	}

    // Run
    benchTimer_start(timer);
    dsps_fft2r_sc16_ansi_(inout_data, data_len, FFT_TABLE_I16);
    benchTimer_stop(timer);

    // Verify
    RezVerifcation ver = RezVerifcation::NoSTD;
    int32_t rez_ind = data_Find_std(FFT_I16_rez_lens, data_len);
    if (rez_ind >= 0){
        ver = RezVerifcation::Match;
        for(int i = 0; i < data_len*2; i++){
            if ( abs(inout_data[i] - FFT_I16_rez[rez_ind][i]) > 2 ){
                ver = RezVerifcation::NoMatch;
            }
        }
    }
    delete(inout_data);
    return ver;
}

RezVerifcation t_iir_biquad_f32_run(int32_t data_len, BenchTimer * timer){
     // Init
    float * in_data = (float *)calloc(data_len, sizeof(float));
    float * rez = (float *)calloc(data_len, sizeof(float));
	for(int i = 0; i < data_len; i++)
	{
		in_data[i] = ((float)data[i]) * 2.15;
	}
    float coef[5] = {0.0738017187,    0.1476034373,    0.0738017187,   -1.2505164146,    0.5457233191};
    float w[2] = {0,0};

    // Run
    benchTimer_start(timer);
    dsps_biquad_f32_ansi(in_data, rez, data_len, coef, w);
    benchTimer_stop(timer);

    // Verify
    RezVerifcation ver = RezVerifcation::NoSTD;
    int32_t rez_ind = data_Find_std(IIR_BQ_rez_lens, data_len);
    if (rez_ind >= 0){
        ver = RezVerifcation::Match;
        for(int i = 0; i < data_len; i++){
            if ( abs(rez[i] - IIR_BQ_rez[rez_ind][i]) > 0.0005 ){
                printf(" %d %f %f ", i, rez[i], IIR_BQ_rez[rez_ind][i]);
                ver = RezVerifcation::NoMatch;
                return ver;
            }
        }
    }
 
    delete(in_data);
    delete(rez);

    return ver;
}

RezVerifcation t_fir_f32_run(int32_t data_len, BenchTimer * timer){
     // Init
    float * in_data = (float *)calloc(data_len, sizeof(float));
    float * rez = (float *)calloc(data_len, sizeof(float));
	for(int i = 0; i < data_len; i++)
	{
		in_data[i] = ((float)data[i]) * 2.15;
	}
    float coeffs[256] = {
        -0.0001080158,   -0.0000223598,   -0.0001508761,    0.0002503345,    0.0002986092,   -0.0002832460,   -0.0002044307,    0.0000756681,   -0.0000781315,    0.0002250069,    0.0003321089,   -0.0003726025,   -0.0003319606,    0.0002122557,    0.0000333043,    0.0001699509, 
        0.0003536868,   -0.0004741130,   -0.0004973665,    0.0004082621,    0.0002157633,    0.0000464696,    0.0003251578,   -0.0005575870,   -0.0006851106,    0.0006671995,    0.0004928094,   -0.0001860992,    0.0001954690,   -0.0005710325,   -0.0008527662,    0.0009656799, 
        0.0008664618,   -0.0005568257,   -0.0000876520,   -0.0004479917,   -0.0009324194,    0.0012483975,    0.0013065514,   -0.0010688850,   -0.0005625123,   -0.0001205478,   -0.0008387699,    0.0014295010,    0.0017448584,   -0.0016874181,   -0.0012373279,    0.0004637551, 
        -0.0004833760,    0.0014011282,    0.0020759818,   -0.0023323157,   -0.0020761732,    0.0013237519,    0.0002067508,    0.0010485509,    0.0021657568,   -0.0028779508,   -0.0029898507,    0.0024283613,    0.0012689550,    0.0002700741,    0.0018666282,   -0.0031606542, 
        -0.0038337166,    0.0036850457,    0.0026863436,   -0.0010012004,    0.0010379463,   -0.0029931566,   -0.0044131008,    0.0049349856,    0.0043737342,   -0.0027771559,   -0.0004320789,   -0.0021834711,   -0.0044950315,    0.0059552171,    0.0061699879,   -0.0049992131, 
        -0.0026069032,   -0.0005538511,   -0.0038224749,    0.0064653359,    0.0078364120,   -0.0075298275,   -0.0054892983,    0.0020467497,   -0.0021236851,    0.0061320813,    0.0090570478,   -0.0101509008,   -0.0090213184,    0.0057471508,    0.0008976353,    0.0045565332, 
        0.0094287251,   -0.0125647387,   -0.0131038832,    0.0106961082,    0.0056238471,    0.0012058492,    0.0084077159,   -0.0143826721,   -0.0176525699,    0.0171987540,    0.0127317770,   -0.0048284243,    0.0051049129,   -0.0150503741,   -0.0227490176,    0.0261604757, 
        0.0239256885,   -0.0157392008,   -0.0025485083,   -0.0134738980,   -0.0291994631,    0.0410213392,    0.0454666661,   -0.0398388516,   -0.0227735814,   -0.0053980079,   -0.0425554054,    0.0849948531,    0.1279133630,   -0.1660979024,   -0.1947083540,    0.2100235390, 
        0.2100235390,   -0.1947083540,   -0.1660979024,    0.1279133630,    0.0849948531,   -0.0425554054,   -0.0053980079,   -0.0227735814,   -0.0398388516,    0.0454666661,    0.0410213392,   -0.0291994631,   -0.0134738980,   -0.0025485083,   -0.0157392008,    0.0239256885, 
        0.0261604757,   -0.0227490176,   -0.0150503741,    0.0051049129,   -0.0048284243,    0.0127317770,    0.0171987540,   -0.0176525699,   -0.0143826721,    0.0084077159,    0.0012058492,    0.0056238471,    0.0106961082,   -0.0131038832,   -0.0125647387,    0.0094287251, 
        0.0045565332,    0.0008976353,    0.0057471508,   -0.0090213184,   -0.0101509008,    0.0090570478,    0.0061320813,   -0.0021236851,    0.0020467497,   -0.0054892983,   -0.0075298275,    0.0078364120,    0.0064653359,   -0.0038224749,   -0.0005538511,   -0.0026069032, 
        -0.0049992131,    0.0061699879,    0.0059552171,   -0.0044950315,   -0.0021834711,   -0.0004320789,   -0.0027771559,    0.0043737342,    0.0049349856,   -0.0044131008,   -0.0029931566,    0.0010379463,   -0.0010012004,    0.0026863436,    0.0036850457,   -0.0038337166, 
        -0.0031606542,    0.0018666282,    0.0002700741,    0.0012689550,    0.0024283613,   -0.0029898507,   -0.0028779508,    0.0021657568,    0.0010485509,    0.0002067508,    0.0013237519,   -0.0020761732,   -0.0023323157,    0.0020759818,    0.0014011282,   -0.0004833760, 
        0.0004637551,   -0.0012373279,   -0.0016874181,    0.0017448584,    0.0014295010,   -0.0008387699,   -0.0001205478,   -0.0005625123,   -0.0010688850,    0.0013065514,    0.0012483975,   -0.0009324194,   -0.0004479917,   -0.0000876520,   -0.0005568257,    0.0008664618, 
        0.0009656799,   -0.0008527662,   -0.0005710325,    0.0001954690,   -0.0001860992,    0.0004928094,    0.0006671995,   -0.0006851106,   -0.0005575870,    0.0003251578,    0.0000464696,    0.0002157633,    0.0004082621,   -0.0004973665,   -0.0004741130,    0.0003536868, 
        0.0001699509,    0.0000333043,    0.0002122557,   -0.0003319606,   -0.0003726025,    0.0003321089,    0.0002250069,   -0.0000781315,    0.0000756681,   -0.0002044307,   -0.0002832460,    0.0002986092,    0.0002503345,   -0.0001508761,   -0.0000223598, -0.000108,
     };
    float delay[256];
    int fir_len = sizeof(coeffs) / sizeof(float);
    fir_f32_t fir1;
    dsps_fir_init_f32(&fir1, coeffs, delay, fir_len);

    // Run
    benchTimer_start(timer);
    dsps_fir_f32_ansi(&fir1, in_data, rez, data_len);
    benchTimer_stop(timer);

    // Verify
    RezVerifcation ver = RezVerifcation::NoSTD;
    int32_t rez_ind = data_Find_std(FIR_rez_lens, data_len);
    if (rez_ind >= 0){
        ver = RezVerifcation::Match;
        for(int i = 0; i < data_len; i++){
            if ( abs(rez[i] - FIR_rez[rez_ind][i]) > 0.0005 ){
                ver = RezVerifcation::NoMatch;
            }
        }
    }

    delete(in_data);
    delete(rez);

    return ver;
}