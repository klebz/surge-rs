crate::ix!();

/**
  | fill the buffer with interleaved stereo
  | samples
  |
  */
fn create_work_buffer(
    l_in:     *mut __m128, 
    r_in:     *mut __m128, 
    nsamples: usize) -> A1d::<__m128> {

    let mut o = A1d::<__m128>::from_elem(HALFRATE_BLOCK_SIZE, z128());

    for k in (0..nsamples).step_by(8) {

        //[o3,o2,o1,o0] = [L0,L0,R0,R0]
        unsafe {

            let l = l_in.add(k >> 3);
            let r = r_in.add(k >> 3);

            o[k + 0]     = _mm_shuffle_ps(*l, *r, _MM_SHUFFLE(0,0,0,0)); 
            o[k + 0 + 1] = _mm_setzero_ps();

            o[k + 2]     = _mm_shuffle_ps(*l, *r, _MM_SHUFFLE(1,1,1,1)); 
            o[k + 2 + 1] = _mm_setzero_ps();

            o[k + 4]     = _mm_shuffle_ps(*l, *r, _MM_SHUFFLE(2,2,2,2)); 
            o[k + 4 + 1] = _mm_setzero_ps();

            o[k + 6]     = _mm_shuffle_ps(*l, *r, _MM_SHUFFLE(3,3,3,3)); 
            o[k + 6 + 1] = _mm_setzero_ps();
        }
    }

    o
}

fn access_lanes(x: *mut f32) -> *mut __m128 {
    x as *mut __m128
}

impl ProcessBlockU2 for crate::HalfRateFilterSSE {

    fn process_block_upsample_by_two(&mut self, 
        l_in: *mut f32, 
        r_in: *mut f32, 
        l: *mut f32, 
        r: *mut f32, 
        nsamples: Option<usize>) 
    {
        let nsamples = nsamples.unwrap_or(64);

        let l = access_lanes(l);
        let r = access_lanes(r);

        let l_in = access_lanes(l_in);
        let r_in = access_lanes(r_in);

        let mut o = create_work_buffer(l_in, r_in, nsamples);

        // process filters
        for j in 0..self.m {

            let mut tx0: __m128 = self.vx0[j];
            let mut tx1: __m128 = self.vx1[j];
            let mut tx2: __m128 = self.vx2[j];
            let mut ty0: __m128 = self.vy0[j];
            let mut ty1: __m128 = self.vy1[j];
            let mut ty2: __m128 = self.vy2[j];
            let ta: __m128 = self.va[j];

            for k in (0..nsamples).step_by(2) {

                // shuffle inputs
                tx2 = tx1;
                tx1 = tx0;
                tx0 = o[k];

                // shuffle outputs
                ty2 = ty1;
                ty1 = ty0;

                // allpass filter 1
                unsafe {
                    ty0 = _mm_add_ps(tx2, _mm_mul_ps(_mm_sub_ps(tx0, ty2), ta));
                }

                o[k] = ty0;

                // shuffle inputs
                tx2 = tx1;
                tx1 = tx0;
                tx0 = o[k + 1];

                // shuffle outputs
                ty2 = ty1;
                ty1 = ty0;

                // allpass filter 1
                unsafe {
                    ty0 = _mm_add_ps(tx2, _mm_mul_ps(_mm_sub_ps(tx0, ty2), ta));
                }

                o[k + 1] = ty0;
            }
            self.vx0[j] = tx0;
            self.vx1[j] = tx1;
            self.vx2[j] = tx2;
            self.vy0[j] = ty0;
            self.vy1[j] = ty1;
            self.vy2[j] = ty2;
        }

        let f_l: *mut f32 = l as *mut f32;
        let f_r: *mut f32 = r as *mut f32;

        let mut fa_r: __m128 = unsafe { _mm_setzero_ps() };
        let mut fb_r: __m128 = unsafe { _mm_setzero_ps() };

        for k in 0..nsamples {

            unsafe {

                let mut v_l: __m128 =  _mm_add_ss(o[k], self.oldout);
                v_l = _mm_mul_ss(v_l, m128_half![]);
                _mm_store_ss(f_l.add(k), v_l);

                fa_r = _mm_movehl_ps(fa_r, o[k]);
                fb_r = _mm_movehl_ps(fb_r, self.oldout);

                let mut v_r: __m128 =  _mm_add_ss(fa_r, fb_r);
                v_r = _mm_mul_ss(v_r, m128_half![]);
                _mm_store_ss(f_r.add(k), v_r);

                self.oldout = _mm_shuffle_ps(o[k], o[k], _MM_SHUFFLE(3, 3, 1, 1));
            }
        }
    }
}
