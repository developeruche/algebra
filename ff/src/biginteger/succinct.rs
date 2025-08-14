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
pub(crate) fn modmul_uint_256<const LIMBS: usize>(
    a: &BigInt<LIMBS>,
    b: &BigInt<LIMBS>,
    modulus: &BigInt<LIMBS>,
) -> BigInt<LIMBS> {
    assert!(LIMBS == BIGINT_WIDTH_WORDS / 2);

    let result_pre = unsafe {
        let mut out = core::mem::MaybeUninit::<[u32; BIGINT_WIDTH_WORDS]>::uninit();
        let a_from = uncompress_4_lib_to_8(&a.0).as_ptr() as *const [u32; BIGINT_WIDTH_WORDS];
        let b_from = uncompress_4_lib_to_8(&b.0).as_ptr() as *const [u32; BIGINT_WIDTH_WORDS];
        let modulus_from = uncompress_4_lib_to_8(&modulus.0).as_ptr() as *const [u32; BIGINT_WIDTH_WORDS];
        
        #[cfg(feature = "std")]
        println!("cycle-tracker-report-start: compute-mul");
        sys_bigint(
            out.as_mut_ptr() as *mut [u32; BIGINT_WIDTH_WORDS],
            OP_MULTIPLY,
            a_from,
            b_from,
            modulus_from,
        );
        #[cfg(feature = "std")]
        println!("cycle-tracker-report-end: compute-mul");
        out.assume_init()
    };

    // performing compression
    let mut result_raw = [0u64; LIMBS];

    for i in 0..LIMBS {
        // Little endian: first u32 is the low bits, second is the high bits
        result_raw[i] = (result_pre[2 * i] as u64) | ((result_pre[2 * i + 1] as u64) << 32);
    }

    let result = BigInt::<LIMBS>::new(result_raw);

    assert!(bool::from(result.lt(&modulus)));
    result
}

/// Uncompresses an array of 4 u64 values into an array of 8 u32 values using little-endian representation.
/// This is the inverse operation of compress_8_lib_to_4.
fn uncompress_4_lib_to_8<const NUM_LIMBS: usize>(input_u64s: &[u64; NUM_LIMBS]) -> [u32; 8] {
    assert_eq!(
        NUM_LIMBS, 4,
        "This function is only designed for NUM_LIMBS=4"
    );
    let mut result_u32s = [0u32; 8];

    for i in 0..NUM_LIMBS {
        // Extract the lower 32 bits
        result_u32s[2 * i] = input_u64s[i] as u32;
        // Extract the higher 32 bits
        result_u32s[2 * i + 1] = (input_u64s[i] >> 32) as u32;
    }

    result_u32s
}

fn compress_8_lib_to_4_le<const NUM_LIMBS: usize>(input_u32s: &[u32; 8]) -> [u64; 4] {
    assert_eq!(
        NUM_LIMBS, 4,
        "This function is only designed for NUM_LIMBS=4"
    );
    let mut result_u64s = [0u64; 4];

    for i in 0..NUM_LIMBS {
        // Little endian: first u32 is the low bits, second is the high bits
        result_u64s[i] = (input_u32s[2 * i] as u64) | ((input_u32s[2 * i + 1] as u64) << 32);
    }

    result_u64s
}