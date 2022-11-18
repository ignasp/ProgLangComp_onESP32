import math
#---------------------------------------------------------------------------------------------------------------------------
#													FFTs 16 bit Fixed Point - dsps_fft_2r_sc16_ansi.c
#													https://github.com/espressif/esp-dsp
#---------------------------------------------------------------------------------------------------------------------------

class fft_2r_sc16_ansi:
    
    add_rount_mult = 0x7fff
    mult_shift_const = 0x7fff # Used to shift data << 15
    
    class sc16_t:
        re = int(0)
        im = int(0)


    def dsp_is_power_of_two(x):
        return ((x != 0) and ((x & (x - 1)) == 0))


    def dsps_fft_2r_sc16_ansi(data, sc_table):
        if fft_2r_sc16_ansi.dsp_is_power_of_two(len(data)) != True :
            return False
        
        cs = fft_2r_sc16_ansi.sc16_t()
        m_data = fft_2r_sc16_ansi.sc16_t()
        a_data = fft_2r_sc16_ansi.sc16_t()

        ie = 1
        N2 = int(len(data)/4)
        
        while(N2 > 0):
            ia = 0
            j = 0
            while j < ie :
                cs.re = sc_table[2*j]
                cs.im = sc_table[2*j+1]

                for i in range(0,N2):
                    m = ia + N2

                    m_data.re = data[2*m]
                    m_data.im = data[2*m+1]     

                    a_data.re = data[2*ia]
                    a_data.im = data[2*ia+1]

                    m1 = fft_2r_sc16_ansi.sc16_t()
                    result = a_data.re*fft_2r_sc16_ansi.mult_shift_const 
                    result -= cs.re*m_data.re + cs.im*m_data.im
                    result += fft_2r_sc16_ansi.add_rount_mult
                    result = result >> 16
                    m1.re = result
                    result = a_data.im *fft_2r_sc16_ansi.mult_shift_const;
                    result -= (cs.re*m_data.im - cs.im*m_data.re)
                    result += fft_2r_sc16_ansi.add_rount_mult
                    result = result >> 16;
                    m1.im = result

                    data[m*2] = m1.re
                    data[m*2+1] = m1.im

                    m2 = fft_2r_sc16_ansi.sc16_t()
                    result = a_data.re *fft_2r_sc16_ansi.mult_shift_const;
                    result += cs.re*m_data.re + cs.im*m_data.im
                    result += fft_2r_sc16_ansi.add_rount_mult
                    result = result >> 16
                    m2.re = result

                    result = a_data.im *fft_2r_sc16_ansi.mult_shift_const
                    result += cs.re*m_data.im - cs.im*m_data.re
                    result += fft_2r_sc16_ansi.add_rount_mult
                    result = result >> 16
                    m2.im = result
                    
                    data[ia*2] = m2.re
                    data[ia*2+1] = m2.im
                    ia += 1
                    
                ia += N2
                j += 1
            ie <<= 1
            N2 >>= 1
        return True
    