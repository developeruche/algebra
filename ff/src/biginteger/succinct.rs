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
    
    let mut result_raw = [0u64; LIMBS];
    
    let a_0: [u32; 8] = unsafe { std::mem::transmute_copy(&a.0) };
    let b_0: [u32; 8] = unsafe { std::mem::transmute_copy(&b.0) };
    let modulus_0: [u32; 8] = unsafe { std::mem::transmute_copy(&modulus.0) };
    
    let result_pre = unsafe {
        let mut out = core::mem::MaybeUninit::<[u32; BIGINT_WIDTH_WORDS]>::uninit();
        sys_bigint(
            out.as_mut_ptr() as *mut [u32; BIGINT_WIDTH_WORDS],
            OP_MULTIPLY,
            &a_0,
            &b_0,
            &modulus_0
        );
        out.assume_init()
    };


    // performing compression
    let result_raw: [u64; LIMBS] = unsafe { std::mem::transmute_copy(&result_pre) };
    let result = BigInt::<LIMBS>::new(result_raw);
    
    result
}