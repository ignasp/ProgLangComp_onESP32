package fir_f32_ansi

type FIR_f32_t struct {
    coeffs	[]float32   /*!< Pointer to the coefficient buffer.*/
    delay	[]float32   /*!< Pointer to the delay line buffer.*/
	N		int32       /*!< FIR filter coefficients amount.*/
    pos		int32       /*!< Position in delay line.*/
    decim	int32       /*!< Decimation factor.*/
    d_pos	int32       /*!< Actual decimation counter.*/
}

func(self * FIR_f32_t) Init(coeffs []float32, delay []float32) bool{
	self.coeffs = coeffs
    self.delay = delay
    self.N = int32(len(coeffs))
    self.pos = 0

    for i := int32(0) ; i < self.N; i++ {
        self.delay[i] = 0
    }
	return true
}

func (self * FIR_f32_t) DSPS_fir_f32_ansi(input []float32, output []float32) bool{

	for i := 0; i < len(input); i++ {
		var acc float32 = 0
		var coeff_pos int32 = self.N - 1
        self.delay[self.pos] = input[i]
        self.pos++;
		if self.pos >= self.N {
            self.pos = 0
        }
		for n := self.pos; n < self.N ; n++ {
            acc += self.coeffs[coeff_pos] * self.delay[n]
			coeff_pos--
        }
        for n := int32(0); n < self.pos; n++ {
            acc += self.coeffs[coeff_pos] * self.delay[n]
			coeff_pos--
        }
        output[i] = acc
	}
	return true
}