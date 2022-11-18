#ifndef CRC32_IEEE_h
#define CRC32_IEEE_h

#include "a_data.h"

void crc32_make_table(uint32_t poly, uint32_t * table);
uint32_t crc32_calc(uint8_t * bytes, uint32_t len, uint32_t * table);

#endif