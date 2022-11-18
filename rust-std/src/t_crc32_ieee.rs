pub mod crc32{
    pub fn make_table(poly: u32) -> [u32; 256] {
        let mut table = [0; 256];
    
        for i in 0..=255 {
            let mut value: u32 = i as u32;      
            // Step through all the bits in the byte
            for _ in 0..8 {
                if (value & 1) != 0 {
                    value = (value >> 1) ^ poly
                } else {
                    value >>= 1
                }
            }
            table[i as usize] = value;
        }
        table
    }
    
    pub fn calc(bytes: &[u8], table: &[u32; 256]) -> u32 {
        let mut value: u32 = 0xffffffff;
        value = bytes.iter().fold(value, |acc, &x| {
            (acc >> 8) ^ (table[((acc ^ (u32::from(x))) & 0xFF) as usize])
        });
        value = !value;
        value
    }
    
    pub fn calc2(bytes: &[u8], table: &[u32; 256], length: usize) -> u32 {
        let mut value: u32 = 0xffffffff;
        for i in 0..length{
            value = table[ (((bytes[i] as u32) ^ value) & 0x000000ff) as usize ] ^ (value >> 8);
        }
        value = !value;
        value
    }
}
