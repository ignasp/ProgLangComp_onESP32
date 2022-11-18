class fir_f32_ansi:
    class fir_st:
        def __init__(self, coeffs, delay):
            self.coeffs = coeffs
            self.delay = delay
            self.N = len(coeffs);
            self.pos = 0;
            for i in range(0, self.N):
                self.delay[i] = 0.0
            
    def dsps_fir_f32_ansi(fir, input, output):
        for i in range(len(input)):
            acc = 0.0
            coeff_pos = fir.N - 1;
            fir.delay[fir.pos] = input[i]
            fir.pos += 1
            if fir.pos >= fir.N:
                fir.pos = 0
            for n in range(fir.pos, fir.N):
                acc += fir.coeffs[coeff_pos] * fir.delay[n]
                coeff_pos -= 1
            for n in range(0, fir.pos):
                acc += fir.coeffs[coeff_pos] * fir.delay[n]
                coeff_pos -= 1
            output[i] = acc