use std::arch::x86_64::*;

pub fn filter_simd_gt_32(buffer: &[i32], threshold_value: i32) -> Vec<usize> {
    let mut results = Vec::new();
    let mut i = 0;
    let len = buffer.len();

    unsafe {
        let cmp = _mm_set1_epi32(threshold_value);

        while i + 4 <= len {
            let ptr = buffer[i..].as_ptr() as *const __m128i;
            let chunk = _mm_loadu_si128(ptr);
            let mask = _mm_cmpgt_epi32(chunk, cmp);
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
        if buffer[j] > threshold_value {
            results.push(j);
        }
    }

    results
}