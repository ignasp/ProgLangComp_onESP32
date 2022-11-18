
#include "t_crc32_ieee.h"

//-------------------------------------------------------------------------------------------------------------------------------
//                                              CRC32 - IEEE
//-------------------------------------------------------------------------------------------------------------------------------
void crc32_make_table(uint32_t poly, uint32_t * table)
{
  for(uint32_t i = 0; i < 256; i++)
  {
    uint32_t byte = i;
    uint32_t crc = 0;
    for(uint32_t bit = 0; bit < 8; bit++)
    {
      if((byte ^ crc) & 1)
      {
        crc = (crc >> 1) ^ poly;
      }
      else
      {
        crc = crc >> 1;
      }
      byte = byte >> 1;
    }
    table[i] = crc;
  }
}

uint32_t crc32_calc(uint8_t * bytes, uint32_t len, uint32_t * table)
{
  uint32_t value = 0xffffffff;
  for(int i = 0; i < len; i++)
  {
    value = table[ (bytes[i] ^ value) & 0x000000ff ] ^ (value >> 8);
  }
  return (uint32_t)(-1 - (int32_t)value);
}
