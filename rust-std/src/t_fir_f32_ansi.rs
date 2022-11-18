pub mod fir_f32_ansi{

    pub struct fir_f32_t<'a>{
        coeffs  : &'a[f32],         /* Pointer to the coefficient buffer.*/
        delay   : &'a mut [f32],    /* Pointer to the delay line buffer.*/
        N       : usize,            /* FIR filter coefficients amount.*/
        pos     : usize,            /* Position in delay line.*/
        decim   : i32,              /* Decimation factor.*/
        d_pos   : usize,            /* Actual decimation counter.*/
    }

    impl<'a> fir_f32_t<'a>{
        pub fn init(coeffs : &'a[f32], delay : &'a mut [f32]) -> fir_f32_t<'a>{
            fir_f32_t{
                coeffs  : coeffs,
                delay   : delay,
                N       : coeffs.len(),
                pos     : 0,
                decim   : 0,
                d_pos   : 0,
            }
        }

        pub fn dsps_fir_f32_ansi(&mut self, input : &[f32], output : &mut [f32]) -> bool{
            for i in 0..input.len() {
                let mut acc = 0.0;
                let mut coeff_pos = (self.N - 1) as i32;
                self.delay[self.pos] = input[i];
                self.pos += 1;
                if self.pos >= self.N {
                    self.pos = 0;
                }
                for n in self.pos..self.N {
                    acc += self.coeffs[coeff_pos as usize] * self.delay[n];
                    coeff_pos -= 1;
                }
                for n in 0..self.pos {
                    acc += self.coeffs[coeff_pos as usize] * self.delay[n];
                    coeff_pos -= 1;
                }
                output[i] = acc;
            }
            return true;
        }
    }

}