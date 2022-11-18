pub mod tests{
    use crate::a_data::data;
    use crate::a_bencher::*;
    
    use crate::t_crc32_ieee::crc32;
    pub fn crc32_run(data_len : usize, timer: &mut my_bench::timer) -> my_bench::rezVerifcation{
        // Init
        let table =crc32::make_table(0xEDB88320);
        let mut in_data = vec![0 as u8; data_len];
        for i in 0..data_len{
            in_data[i] = data::DATA[i];
        }
        
        // Run
        timer.start();
        let out_data = crc32::calc2(&in_data, &table, data_len);
        timer.stop();
        

        // Veriy
        let rez_ind = data::find_std(&data::CRC32_rez_lens, data_len);
        if rez_ind < 0 {
            return my_bench::rezVerifcation::NoStd;
        }
        else{
            let std = &data::CRC32_rez[rez_ind as usize];
            if out_data != std[0] {
                return my_bench::rezVerifcation::NoMatch
            }
            return my_bench::rezVerifcation::Match;
        }
    }

    use crate::t_fft_2r_sc16_ansi::fft_2r_sc16_ansi::*;
    pub fn fft_2r_sc16_ansi_run(data_len : usize, timer: &mut my_bench::timer) -> my_bench::rezVerifcation{
        
        // Init
        let mut inout_data = vec![0 as i16; data_len*2];
        for i in 0..data_len*2{
            inout_data[i] = data::DATA[i] as i16;
        }
        
        // Run
        timer.start();
        dsps_fft2r_sc16_ansi2(&mut inout_data, &FFT_TABLE);
        timer.stop();

        // Veriy
        let rez_ind = data::find_std(&data::FFT_I16_rez_lens, data_len);
        if rez_ind < 0 {
            return my_bench::rezVerifcation::NoStd;
        }
        else{
            let std = &data::FFT_I16_rez[rez_ind as usize];
            for i in 0..inout_data.len(){
                let diff = (inout_data[i] - std[i]).abs();
                if diff > 2{
                    return my_bench::rezVerifcation::NoMatch
                }
            }
            return my_bench::rezVerifcation::Match;
        }
    }

    use crate::t_sha256::sha256::*;
    pub fn sha256_run(data_len : usize, timer: &mut my_bench::timer) -> my_bench::rezVerifcation{
        
        // Init
        let mut in_data = vec![0; data_len];
        for i in 0..data_len{
            in_data[i] = data::DATA[i] as BYTE;
        }
        let mut ctx = Sha256Ctx{
            data:       [0; 64],
            datalen:    0,
            bitlen:     0,
            state:      [0; 8],
        };
        let mut rez: [BYTE;SHA256_BLOCK_SIZE] = [0;SHA256_BLOCK_SIZE];
        
        // Run
        timer.start();
        sha256_init(&mut ctx);
        sha256_update(&mut ctx, &in_data);
	    sha256_final(&mut ctx, &mut rez);
        timer.stop();

        // Verify
        let rez_ind = data::find_std(&data::SHA256_rez_lens, data_len);
        if rez_ind < 0 {
            return my_bench::rezVerifcation::NoStd;
        }
        else{
            let std = &data::SHA256_rez[rez_ind as usize];
            for i in 0..rez.len(){
                if  rez[i] != std[i] {
                    return my_bench::rezVerifcation::NoMatch
                }
            }
            return my_bench::rezVerifcation::Match;
        }

    }

    use crate::t_iir_biquad_f32_ansi::iir_biquad_f32_ansi::*;
    pub fn iir_biquad_f32_ansi_run(data_len : usize, timer: &mut my_bench::timer) -> my_bench::rezVerifcation{
        
        // Init
        let mut in_data = vec![0 as f32; data_len];
        for i in 0..data_len{
            in_data[i] = (data::DATA[i] as f32) * 2.15;
        }
        let mut rez = vec![0 as f32; data_len];

        let coef : [f32;5] = [0.0738017187,    0.1476034373,    0.0738017187,   -1.2505164146,    0.5457233191];
        let mut w : [f32;2] = [0.0, 0.0];
        
        // Run
        timer.start();
        dsps_iir_biquad_f32_ansi(&in_data, &mut rez, &coef, &mut w);
        timer.stop();

        // Veriy
        let rez_ind = data::find_std(&data::IIR_BQ_rez_lens, data_len);
        if rez_ind < 0 {
            return my_bench::rezVerifcation::NoStd;
        }
        else{
            let std = &data::IIR_BQ_rez[rez_ind as usize];
            for i in 0..rez.len(){
                let diff = (rez[i] - std[i]).abs();
                if diff > 0.0005{
                    return my_bench::rezVerifcation::NoMatch
                }
            }
            return my_bench::rezVerifcation::Match;
        }
    }


    use crate::t_fir_f32_ansi::fir_f32_ansi::*;
    pub fn fir_f32_ansi_run(data_len : usize, timer: &mut my_bench::timer) -> my_bench::rezVerifcation{
        
        // Init
        let mut in_data = vec![0 as f32; data_len];
        for i in 0..data_len{
            in_data[i] = (data::DATA[i] as f32) * 2.15;
        }
        let mut rez = vec![0 as f32; data_len];

        const coef : [f32; 256] = [
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
        ];
        let mut delay  = [0.0; coef.len()];
        let mut fir1 = fir_f32_t::init(&coef, &mut delay);

        
        // Run
        timer.start();
        fir1.dsps_fir_f32_ansi(&in_data, &mut rez);
        timer.stop();

        // Veriy
        let rez_ind = data::find_std(&data::FIR_rez_lens, data_len);
        if rez_ind < 0 {
            return my_bench::rezVerifcation::NoStd;
        }
        else{
            let std = &data::FIR_rez[rez_ind as usize];
            for i in 0..rez.len(){
                let diff = (rez[i] - std[i]).abs();
                if diff > 0.0005{
                    return my_bench::rezVerifcation::NoMatch
                }
            }
            return my_bench::rezVerifcation::Match;
        }
    }
}