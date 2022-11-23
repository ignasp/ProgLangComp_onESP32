import sys
import gc
sys.path.append("./a_data")

def t_OVERHEAD_run(data_len, timer):
    timer.start()
    timer.stop()

def t_CRC32_run(data_len, timer):
    from a_in_data import data
   
    from a_bencher import RezVerifcation
    
    from t_crc32_Ieee import crc32_ieee as crc32
    from a_crc32_rez import CRC32_rez_data as rez_data
    
    
    table = crc32.make_table(0xEDB88320)
    
    timer.start()
    rez = crc32.calc(data.DATA[0:data_len], table)
    timer.stop()
    
    ver = RezVerifcation.NoStd
    if data_len in rez_data.rez_lens:
        ver = RezVerifcation.NoMatch
        rez_ind = rez_data.rez_lens.index(data_len)
        if rez == rez_data.rez[rez_ind][0]:
            ver = RezVerifcation.Match
            
    del data, RezVerifcation
    del rez_data, crc32
    gc.collect()
    
    return ver


def t_SHA256_run(data_len, timer):
    from a_in_data import data
    from a_bencher import RezVerifcation
    
    from t_sha256 import sha256
    from a_sha256_rez import SHA256_rez_data as rez_data
    
    ctx_v = sha256.CTX()
    rez = [0]*sha256.BLOCK_SIZE
    
    timer.start()
    sha256.initialize(ctx_v)
    sha256.update(ctx_v, data.DATA[0:data_len])
    sha256.final(ctx_v, rez)
    timer.stop()
    
    ver = RezVerifcation.NoStd
    if data_len in rez_data.rez_lens:
        ver = RezVerifcation.Match
        rez_ind = rez_data.rez_lens.index(data_len)
        i = 0;
        for val in rez_data.rez[rez_ind]:
            if rez[i] != val:
                ver = RezVerifcation.NoMatch
                break
            i += 1
            
    del data, RezVerifcation
    del sha256, rez_data
    gc.collect()
    
    return ver

def t_FFT_I16_run(data_len, timer):
    from a_in_data import data
    from a_fft_i16_table import FFT_I16_TABLE
    from a_bencher import RezVerifcation
    
    from t_fft_2r_sc16_ansi import fft_2r_sc16_ansi as fft
    
    inout_data = [0]*(data_len*2)
    for i in range(0, data_len*2):
        inout_data[i] = data.DATA[i];
        
    del data
    gc.collect()
    
    timer.start()
    fft.dsps_fft_2r_sc16_ansi(inout_data, FFT_I16_TABLE.table)
    timer.stop()
    
    del FFT_I16_TABLE
    gc.collect()
    
    from a_fft_i16_rez import FFT_I16_rez_data as rez_data
    ver = RezVerifcation.NoStd
    if data_len in rez_data.rez_lens:
        ver = RezVerifcation.Match
        rez_ind = rez_data.rez_lens.index(data_len)
        i = 0;
        for val in inout_data[0:32] :
            if abs(rez_data.rez[rez_ind][i] - val) > 2:
                ver = RezVerifcation.NoMatch
                break
            i += 1
            
    del RezVerifcation, rez_data
    del fft
    gc.collect()
    
    return ver



def t_iir_biquad_f32_run(data_len, timer):
    from a_in_data import data
    from a_bencher import RezVerifcation
    
    from t_iir_biquad_ansi import iir_biquad_ansi as iir
    
    in_data = [0.0]*(data_len)
    rez = [0.0]*(data_len)
    coef = [0.0738017187,    0.1476034373,    0.0738017187,   -1.2505164146,    0.5457233191]
    w = [0.0, 0.0]
    for i in range(0, data_len):
        in_data[i] = data.DATA[i] * 2.15;
    del data
    gc.collect()
    
    timer.start()
    iir.dsps_biquad_f32_ansi(in_data, rez, coef, w)
    timer.stop()
    
    from a_iir_f32_rez import IIR_F32_rez_data as rez_data
    ver = RezVerifcation.NoStd
    if data_len in rez_data.rez_lens:
        ver = RezVerifcation.Match
        rez_ind = rez_data.rez_lens.index(data_len)
        i = 0;
        for val in rez[0:32] :
            if abs(rez_data.rez[rez_ind][i] - val) > 0.0005:
                ver = RezVerifcation.NoMatch
                break
            i += 1
            
    del RezVerifcation, rez_data
    del iir
    gc.collect()
    
    return ver

def t_fir_f32_run(data_len, timer):
    from a_in_data import data
    from a_bencher import RezVerifcation
    
    from t_fir_f32_ansi import fir_f32_ansi as fir
    
    in_data = [0.0]*(data_len)
    rez = [0.0]*(data_len)
    for i in range(0, data_len):
        in_data[i] = data.DATA[i] * 2.15;
    coeffs = [
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
    ]
    delay = [0.0]*256
        
    fir1 = fir.fir_st(coeffs, delay)

    del data
    gc.collect()
    
    timer.start()
    fir.dsps_fir_f32_ansi(fir1, in_data, rez)
    timer.stop()
    
    from a_fir_f32_rez import FIR_F32_rez_data as rez_data
    ver = RezVerifcation.NoStd
    if data_len in rez_data.rez_lens:
        ver = RezVerifcation.Match
        rez_ind = rez_data.rez_lens.index(data_len)
        i = 0;
        for val in rez[0:32] :
            if abs(rez_data.rez[rez_ind][i] - val) > 0.0005:
                ver = RezVerifcation.NoMatch
                break
            i += 1
            
    del RezVerifcation, rez_data
    del fir
    gc.collect()
    
    return ver
