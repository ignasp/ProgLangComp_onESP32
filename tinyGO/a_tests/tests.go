package tests

import (
	//"fmt"
	"math"
	bench "tinyGO/a_bencher"
	//data "tinyGO/a_data"
	data "tinyGO/a_data2"
	crc32 "tinyGO/t_crc32_ieee"
	ffti16 "tinyGO/t_fft2r_sc16_ansi"
	sha256 "tinyGO/t_sha256"
	fir_f32 "tinyGO/t_fir_f32_ansi"
	iir_f32 "tinyGO/t_iir_biquad_f32_ansi"
)

func CRC32_run(data_len uint32, timer *bench.BenchTimer) bench.RezVerifcation {
	// Init
	var table crc32.Crc_table
	crc32.Make_table(0xEDB88320, &table)
	in_data := make([]byte, data_len)
	for i := uint32(0); i < data_len; i++ {
		in_data[i] = data.DATA[i]
	}

	// Run
	timer.Start()
	rez := crc32.Calc(in_data, &table)
	timer.Stop()

	//Verify
	std_i := data.Find_std(data.CRC32_rez_lens, data_len)
	if std_i < 0 {
		return bench.NoStd
	} else {
		if rez == data.CRC32_rez[std_i][0] {
			return bench.Match
		}
		return bench.NoMatch
	}
}

func SHA256_run(data_len uint32, timer *bench.BenchTimer) bench.RezVerifcation {
	// Init
	var ctx_v sha256.SHA256_CTX
	var rez [sha256.SHA256_BLOCK_SIZE]byte
	in_data := make([]byte, data_len)
	for i := uint32(0); i < data_len; i++ {
		in_data[i] = data.DATA[i]
	}

	// Run
	timer.Start()

	sha256.Init(&ctx_v)
	sha256.Update(&ctx_v, in_data)
	sha256.Final(&ctx_v, &rez)

	timer.Stop()

	// Verify
	std_i := data.Find_std(data.SHA256_rez_lens, data_len)
	if std_i < 0 {
		return bench.NoStd
	} else {
		for i, v := range data.SHA256_rez[std_i] {
			if v != rez[i] {
				return bench.NoMatch
			}
		}
		return bench.Match
	}
}

func FFT_I16_run(data_len uint32, timer *bench.BenchTimer) bench.RezVerifcation {
	// Init
	inout_data := make([]int16, data_len*2)
	for i := uint32(0); i < data_len*2; i++ {
		inout_data[i] = int16(data.DATA[i])
	}

	// Run
	timer.Start()
	ffti16.Dsps_fft2r_sc16_ansi2_(inout_data, ffti16.FFT_table)
	timer.Stop()

	// Veriy
	std_i := data.Find_std(data.FFT_I16_rez_lens, data_len)
	if std_i < 0 {
		return bench.NoStd
	} else {
		for i := uint32(0); i < 30 && i < data_len*2; i++{ 
		//for i := uint32(0); i < data_len*2; i++{
			if math.Abs( float64(data.FFT_I16_rez[std_i][i] - inout_data[i]) ) > 2 {
				return bench.NoMatch
			}
		}
		return bench.Match
	}
}


func FIR_F32_run(data_len uint32, timer *bench.BenchTimer) bench.RezVerifcation {
	// Init
	in_data := make([]float32, data_len)
	rez := make([]float32, data_len)
	for i := uint32(0); i < data_len; i++ {
		var p = data.DATA[i];
		in_data[i] = float32(p) * 2.15
		rez[i] = 0;
	}
	var coeffs = [256]float32{
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
	var delay [256]float32
	var fir1 fir_f32.FIR_f32_t
	fir1.Init(coeffs[:], delay[:])

	// Run
	timer.Start()
	fir1.DSPS_fir_f32_ansi(in_data, rez)
	timer.Stop()

	// Veriy
	std_i := data.Find_std(data.FIR_rez_lens, data_len)
	if std_i < 0 {
		return bench.NoStd
	} else { 
		std := data.FIR_rez[std_i]
		//for i := uint32(0); i < data_len; i++{
		for i := uint32(0); i < 32 && i < data_len; i++{
			if math.Abs( float64(std[i] - rez[i]) ) > 0.0005 {
				return bench.NoMatch
			}
		}
		return bench.Match
	}
}

func IIR_F32_run(data_len uint32, timer *bench.BenchTimer) bench.RezVerifcation {
	// Init
	in_data := make([]float32, data_len)
	rez := make([]float32, data_len)
	for i := uint32(0); i < data_len; i++ {
		var p = data.DATA[i];
		in_data[i] = float32(p) * 2.15
		rez[i] = 0;
	}
	var coeffs = [5]float32{0.0738017187,    0.1476034373,    0.0738017187,   -1.2505164146,    0.5457233191}
	var w = [2]float32{0, 0}

	// Run
	timer.Start()
	iir_f32.DSPS_iir_biquad_f32_ansi(in_data, rez, coeffs, w)
	timer.Stop()

	// Veriy
	std_i := data.Find_std(data.IIR_BQ_rez_lens, data_len)
	if std_i < 0 {
		return bench.NoStd
	} else { 
		std := data.IIR_BQ_rez[std_i]
		//for i := uint32(0); i < data_len; i++{
		for i := uint32(0); i < 32 && i < data_len; i++{
			if math.Abs( float64(std[i] - rez[i]) ) > 0.0005 {
				return bench.NoMatch
			}
		}
		return bench.Match
	}
}
