#from a_data import data
from array import array
class crc32_ieee:
    #------------------------------------------------------------------------------------------------
    #                                              CRC32 - IEEE
    #------------------------------------------------------------------------------------------------
    
    table = 0
    rez = 0
    data_len = 0
    
    def make_table(poly):
        table = array('L')
        for byte in range(256):
            crc = 0
            for bit in range(8):
                if (byte ^ crc) & 1:
                    crc = (crc >> 1) ^ poly
                else:
                    crc >>= 1
                byte >>= 1
            table.append(crc)
        return table


    def calc(bytes, table):
        value = 0xffffffff
        for ch in bytes:
            value = table[(ch ^ value) & 0xff] ^ (value >> 8)
        return (-1 - value) & 0xFFFFFFFF
