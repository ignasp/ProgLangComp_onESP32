package iir_biquar_f32_ansi

func DSPS_iir_biquad_f32_ansi(input []float32, output []float32, coef[5]float32, w[2]float32) bool{
	for i := 0; i < len(input); i++ {
        d0 := input[i] - coef[3] * w[0] - coef[4] * w[1]
        output[i] = coef[0] * d0 +  coef[1] * w[0] + coef[2] * w[1]
        w[1] = w[0]
        w[0] = d0
    }
	return true
}