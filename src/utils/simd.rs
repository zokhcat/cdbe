use std::arch::x86_64::*;

pub enum SimdOp {
    Eq,
    Lt,
    Gt,
    Le,
    Ge,
}

pub fn filter_simd_32(buffer: &[i32], threshold_value: i32, op: SimdOp) -> Vec<usize> {
    let mut results: Vec<usize> = Vec::new();
    let mut i = 0;
    let len = buffer.len();

    unsafe {
        let cmp = _mm_set1_epi32(threshold_value);

        while i + 4 <= len {
            let ptr = buffer[i..].as_ptr() as *const __m128i;
            let chunk = _mm_loadu_si128(ptr);
            let mask = match op {
                SimdOp::Eq => _mm_cmpeq_epi32(chunk, cmp),
                SimdOp::Lt => _mm_cmplt_epi32(chunk, cmp),
                SimdOp::Gt => _mm_cmpgt_epi32(chunk, cmp),
                SimdOp::Le => {
                    let gt = _mm_cmpgt_epi32(chunk, cmp);
                    _mm_cmpeq_epi32(_mm_setzero_si128(), gt)
                }
                SimdOp::Ge => {
                    let lt = _mm_cmpgt_epi32(cmp, chunk);
                    _mm_cmpeq_epi32(_mm_setzero_si128(), lt)
                }
            };

            let mask_bits = _mm_movemask_ps(std::mem::transmute(mask));

            for j in 0..4 {
                if (mask_bits & (1 << j)) != 0 {
                    results.push(i + j);
                }
            }

            i += 4;
        }
    }

    for j in i..len {
        let matched = match op {
            SimdOp::Eq => buffer[j] == threshold_value,
            SimdOp::Lt => buffer[j] < threshold_value,
            SimdOp::Gt => buffer[j] > threshold_value,
            SimdOp::Le => buffer[j] <= threshold_value,
            SimdOp::Ge => buffer[j] >= threshold_value,
        };
        if matched {
            results.push(j);
        }
    }

    results
}