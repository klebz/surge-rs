crate::ix!();

#[test] fn vocoder_smoke() {

    const N: usize = 2048;

    let l: Vec<f32> = (0..N).map(|x| surge_math::correlated_noise(0.0, x as f64 / 16.0) as f32).collect();
    let r: Vec<f32> = (0..N).map(|x| surge_math::correlated_noise(0.0, x as f64 / 16.0) as f32).collect();

    println!("l: {:?}",l); 
    println!("r: {:?}",r); 

    let srunit    = SampleRateHandle::default();
    let tables    = TablesHandle::new(&srunit);
    let synth_in  = SynthInputHandle::default();

    let mut x     = crate::Vocoder::new::<N>(&tables, &srunit, &synth_in);
    x.init();

    /*TODO: fix synth_in memory access error
    x.params[crate::VocoderParam::Rate].val = PData::Float(1.0);

    for iter in 0..24{
        x.process(l.as_mut_ptr(), r.as_mut_ptr());
        println!("l: {:?}",l); 
        println!("r: {:?}",r); 
    }
    */
}
