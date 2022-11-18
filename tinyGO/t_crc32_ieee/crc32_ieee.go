package crc32_ieee

//------------------------------------------------------------------------------------------------
//                                              CRC32 - IEEE
//------------------------------------------------------------------------------------------------

type Crc_table [256]uint32

func Make_table(poly uint32, t *Crc_table) {
	for i := 0; i < 256; i++ {
		crc := uint32(i)
		for j := 0; j < 8; j++ {
			if crc & 1 == 1 {
				crc = (crc >> 1) ^ poly
			} else {
				crc >>= 1
			}
		}
		t[i] = crc
	}
}

func Calc(p []byte, tab *Crc_table) uint32 {
	var crc = uint32(0xffffffff)
	for _, v := range p {
		crc = tab[byte(crc)^v] ^ (crc >> 8)
	}
	return ^crc
}
