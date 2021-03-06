crate::ix!();

#[test] fn freqshift_smoke() {

    const N: usize = 32;

    let mut l: Vec<f32> = (0..N).map(|x| surge_math::correlated_noise(0.0, x as f64 / 16.0) as f32).collect();
    let mut r: Vec<f32> = (0..N).map(|x| surge_math::correlated_noise(0.0, x as f64 / 16.0) as f32).collect();

    println!("l: {:?}",l); 
    println!("r: {:?}",r); 

    let srunit    = SampleRateHandle::default();
    let tuner     = TunerHandle::new(&srunit);
    let tables    = TablesHandle::new(&srunit);
    let timeunit  = TimeUnitHandle::new(&srunit);

    let mut x     = FreqShift::new(&tuner, &tables, &srunit, &timeunit);

    x.params[FreqShiftParam::Feedback].val = PData::Float(0.7);

    for _ in 0..24{

        x.process::<N>(
            l.as_mut_slice().try_into().unwrap(), 
            r.as_mut_slice().try_into().unwrap()
        );

        println!("l: {:?}",l); 
        println!("r: {:?}",r); 
    }
}

