crate::ix!();

impl Chorus {

    pub fn do_chorus_block<const N: usize>(&mut self, 
        k:     usize, 
        out_l: *mut f32,
        out_r: *mut f32
    ) {

        let mut l: __m128 = unsafe{ _mm_setzero_ps() };
        let mut r: __m128 = unsafe{ _mm_setzero_ps() };  

        for j in 0..CHORUS_DEPTH {

            self.time[j].process();

            let vtime: f32 = self.time[j].v;

            let i_dtime: i32 = 
                std::cmp::max(
                    N as i32, 
                    std::cmp::min(
                        vtime as i32, 
                        (CHORUS_MAX_DELAY_LENGTH - FIR_IPOL_N - 1) as i32
                    )
                );

            let rp: usize = 
                ((((self.wpos as isize) - (i_dtime as isize) + k as isize) - FIR_IPOL_N as isize) & 
                ((CHORUS_MAX_DELAY_LENGTH - 1) as isize)).try_into().unwrap();

            let sinc: usize = FIR_IPOL_N * 
                limit_range(
                    (((FIR_IPOL_M as f32) * ((i_dtime + 1) as f32 - vtime)) as i32) as f32, 
                    0.0, 
                    (FIR_IPOL_M - 1) as f32
                ) as usize;

            unsafe {
                let mut vo: __m128 = _mm_mul_ps(
                    _mm_load_ps(self.tables.sinctable_1x_ptr(sinc)),     
                    _mm_loadu_ps(self.buffer.as_mut_ptr().add(rp))
                );
                vo = _mm_add_ps( vo, 
                    _mm_mul_ps(
                        _mm_load_ps(self.tables.sinctable_1x_ptr(sinc + 4)), 
                        _mm_loadu_ps(self.buffer.as_mut_ptr().add(rp + 4))
                    )
                );
                vo = _mm_add_ps( 
                    vo, 
                    _mm_mul_ps(
                        _mm_load_ps(self.tables.sinctable_1x_ptr(sinc + 8)), 
                        _mm_loadu_ps(self.buffer.as_mut_ptr().add(rp + 8))
                    )
                );

                l = _mm_add_ps(l, _mm_mul_ps(vo, self.voicepan_l4[j]));
                r = _mm_add_ps(r, _mm_mul_ps(vo, self.voicepan_r4[j]));
            }
        }

        unsafe {
            l = sum_ps_to_ss(l);
            r = sum_ps_to_ss(r);
            _mm_store_ss(out_l.add(k), l);
            _mm_store_ss(out_r.add(k), r);
        }
    }
}
