use crate::BigInt;

extern "C" {
    /// Computes a big integer operation with a modulus.
    pub fn sys_bigint(
        result: *mut [u32; 8],
        op: u32,
        x: *const [u32; 8],
        y: *const [u32; 8],
        modulus: *const [u32; 8],
    );
}

pub const BIGINT_WIDTH_WORDS: usize = 8;
const OP_MULTIPLY: u32 = 0;

#[inline(always)]
pub(crate) fn modmul_uint_256(a: &BigInt<4>, b: &BigInt<4>, modulus: &BigInt<4>) -> BigInt<4> {
    assert!(4 == BIGINT_WIDTH_WORDS);

    let result_raw = unsafe {
        let mut out = core::mem::MaybeUninit::<[u32; LIMBS]>::uninit();
        sys_bigint(
            out.as_mut_ptr() as *mut [u32; BIGINT_WIDTH_WORDS],
            OP_MULTIPLY,
            a.as_words().as_ptr() as *const [u32; BIGINT_WIDTH_WORDS],
            b.as_words().as_ptr() as *const [u32; BIGINT_WIDTH_WORDS],
            modulus.as_words().as_ptr() as *const [u32; BIGINT_WIDTH_WORDS],
        );
        out.assume_init()
    };
    let result = BigInt::<4>::new(compress_8_lib_to_4_le(&result_raw));
    assert!(bool::from(result.lt(&modulus)));
    result
}

fn compress_8_lib_to_4_le(input: &[u32; 8]) -> [u64; 4] {
    let mut result = [0u64; 4];

    for i in 0..4 {
        // Little endian: first u32 is the low bits, second is the high bits
        result[i] = (input[2 * i] as u64)
            | ((input[2 * i + 1] as u64) << 32);
    }

    result
}
