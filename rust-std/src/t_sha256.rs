pub mod sha256{
    //------------------------------------------------------------------------------------------------
	//                                           SHA256
	//    https://github.com/B-Con/crypto-algorithms/blob/master/sha256.c
	//    https://github.com/RustCrypto/hashes/tree/master/sha2/src
	//--------------------------------------------------------------------------------------------

	pub const SHA256_BLOCK_SIZE: usize = 32;

	pub type BYTE = u8;
	pub type WORD = u32;

	pub struct Sha256Ctx {
		pub data:       [BYTE; 64],
		pub datalen:    usize,
		pub bitlen:     u64,
		pub state:      [WORD; 8],
	}

	//**************************** VARIABLES *****************************/
	/// Constants necessary for SHA-256 family of digests.
	pub const K: [u32; 64] = [
		0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
		0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
		0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
		0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
		0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
		0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
		0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
		0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
	];

	/****************************** MACROS ******************************/

	macro_rules! CH {
			($a:expr, $b:expr, $c:expr) => {
				($a & $b) ^ (!$a & $c)
			};
		}
		
	macro_rules! MAJ {
			($a:expr, $b:expr, $c:expr) => {
				($a & $b) ^ ($a & $c) ^ ($b & $c)
			};
		}

	macro_rules! EP0 {
			($a:expr) => {
				($a.rotate_right(2) ^ $a.rotate_right(13) ^ $a.rotate_right(22))
			};
		}
		
	macro_rules! EP1 {
			($a:expr) => {
				($a.rotate_right(6) ^ $a.rotate_right(11) ^ $a.rotate_right(25))
			};
		}

	macro_rules! SIG0 {
			($a:expr) => {
				($a.rotate_right(7) ^ $a.rotate_right(18) ^ ($a >> 3))
			};
		}
		
	macro_rules! SIG1 {
			($a:expr) => {
				($a.rotate_right(17) ^ $a.rotate_right(19) ^ ($a >> 10))
			};
		}

	/*********************** FUNCTION DEFINITIONS ***********************/

	pub fn sha256_transform(ctx: &mut Sha256Ctx, data: [BYTE;64])
	{
		let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h, mut t1, mut t2): (WORD, WORD, WORD, WORD, WORD, WORD, WORD, WORD, WORD, WORD);
		let (mut i, mut j): (usize, usize);
		let mut m : [WORD;64] = [0;64];

		i = 0;
		j = 0;
		while i < 16
		{
			m[i] = ((data[j] as WORD).wrapping_shl(24) | (data[j + 1] as WORD).wrapping_shl(16) | (data[j + 2] as WORD).wrapping_shl(8) | (data[j + 3] as WORD)) as WORD;
			i += 1;
			j += 4;
		}
		while i < 64
		{
			m[i] = SIG1!(m[i - 2]).wrapping_add(m[i - 7]).wrapping_add(SIG0!(m[i - 15])).wrapping_add(m[i - 16]) as WORD;
			i += 1;
		}
		

		a = ctx.state[0];
		b = ctx.state[1];
		c = ctx.state[2];
		d = ctx.state[3];
		e = ctx.state[4];
		f = ctx.state[5];
		g = ctx.state[6];
		h = ctx.state[7];

		for i in 0..64 
		{
			t1 = h.wrapping_add(EP1!(e)).wrapping_add(CH!(e,f,g)).wrapping_add(K[i]).wrapping_add(m[i]);
			t2 = EP0!(a).wrapping_add(MAJ!(a,b,c));
			h = g;
			g = f;
			f = e;
			e = d.wrapping_add(t1);
			d = c;
			c = b;
			b = a;
			a = t1.wrapping_add(t2);
		}


		ctx.state[0] = ctx.state[0].wrapping_add(a);
		ctx.state[1] = ctx.state[1].wrapping_add(b);
		ctx.state[2] = ctx.state[2].wrapping_add(c);
		ctx.state[3] = ctx.state[3].wrapping_add(d);
		ctx.state[4] = ctx.state[4].wrapping_add(e);
		ctx.state[5] = ctx.state[5].wrapping_add(f);
		ctx.state[6] = ctx.state[6].wrapping_add(g);
		ctx.state[7] = ctx.state[7].wrapping_add(h);
	}

	pub fn sha256_init(ctx: &mut Sha256Ctx)
	{
		ctx.datalen = 0;
		ctx.bitlen = 0;
		ctx.state[0] = 0x6a09e667;
		ctx.state[1] = 0xbb67ae85;
		ctx.state[2] = 0x3c6ef372;
		ctx.state[3] = 0xa54ff53a;
		ctx.state[4] = 0x510e527f;
		ctx.state[5] = 0x9b05688c;
		ctx.state[6] = 0x1f83d9ab;
		ctx.state[7] = 0x5be0cd19;
	}

	pub fn sha256_update(ctx: &mut Sha256Ctx, data: &[BYTE])
	{
		for i in data
		{
			ctx.data[ctx.datalen] = *i;
			ctx.datalen += 1;
			if ctx.datalen == 64 
			{
				sha256_transform(ctx, ctx.data);
				ctx.bitlen += 512;
				ctx.datalen = 0;
			}
		}
	}


	pub fn sha256_final(ctx :&mut Sha256Ctx, hash: &mut[BYTE; SHA256_BLOCK_SIZE])
	{
		let mut i = ctx.datalen;

		// Pad whatever data is left in the buffer.
		if ctx.datalen < 56 
		{
			ctx.data[i] = 0x80;
			i += 1;
			while i < 56
			{
				ctx.data[i] = 0x00;
				i += 1;
			}
		}
		else 
		{
			ctx.data[i] = 0x80;
			i += 1;
			while i < 64
			{
				ctx.data[i] = 0x00;
				i = 1;
			}
			sha256_transform(ctx, ctx.data);
			for j in &mut ctx.data[0..56] { *j = 0; }
		}

		// Append to the padding the total message's length in bits and transform.
		ctx.bitlen += (ctx.datalen * 8) as u64;
		ctx.data[63] = (ctx.bitlen) as BYTE;
		ctx.data[62] = (ctx.bitlen >> 8) as BYTE;
		ctx.data[61] = (ctx.bitlen >> 16) as BYTE;
		ctx.data[60] = (ctx.bitlen >> 24) as BYTE;
		ctx.data[59] = (ctx.bitlen >> 32) as BYTE;
		ctx.data[58] = (ctx.bitlen >> 40) as BYTE;
		ctx.data[57] = (ctx.bitlen >> 48) as BYTE;
		ctx.data[56] = (ctx.bitlen >> 56) as BYTE;
		sha256_transform(ctx, ctx.data);

		// Since this implementation uses little endian byte ordering and SHA uses big endian,
		// reverse all the bytes when copying the final state to the output hash.
		for i in 0..4
		{
			hash[i]      = ((ctx.state[0] >> (24 - i * 8)) & 0x000000ff) as BYTE;
			hash[i + 4]  = ((ctx.state[1] >> (24 - i * 8)) & 0x000000ff) as BYTE;
			hash[i + 8]  = ((ctx.state[2] >> (24 - i * 8)) & 0x000000ff) as BYTE;
			hash[i + 12] = ((ctx.state[3] >> (24 - i * 8)) & 0x000000ff) as BYTE;
			hash[i + 16] = ((ctx.state[4] >> (24 - i * 8)) & 0x000000ff) as BYTE;
			hash[i + 20] = ((ctx.state[5] >> (24 - i * 8)) & 0x000000ff) as BYTE;
			hash[i + 24] = ((ctx.state[6] >> (24 - i * 8)) & 0x000000ff) as BYTE;
			hash[i + 28] = ((ctx.state[7] >> (24 - i * 8)) & 0x000000ff) as BYTE;
		}
	}
}
