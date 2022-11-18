pub mod iir_biquad_f32_ansi{

    pub fn dsps_iir_biquad_f32_ansi(input : &[f32], output : &mut [f32], coef : &[f32;5], w : &mut [f32;2]) -> bool{
        for i in 0..input.len() {
            let d0 = input[i] - coef[3] * w[0] - coef[4] * w[1];
            output[i] = coef[0] * d0 +  coef[1] * w[0] + coef[2] * w[1];
            w[1] = w[0];
            w[0] = d0;
        }
        return true;
    }

}