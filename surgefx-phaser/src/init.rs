ix!();

use crate::{
    Phaser,
    PhaserParam,
};

impl Init for Phaser<'sr> {

    fn init(&mut self) {
        self.lfophase = 0.25;
        self.suspend_all_biquads();
        self.clear_blocks();
        self.mix.set_target(1.0);
        self.mix.instantize();
        self.bi = 0;
        self.d_l = 0.0;
        self.d_r = 0.0;
    }
}

impl Phaser<'sr> {

    pub fn new(
        tuner:     &'sr TunerHandle<'sr>,
        tables:    &'sr TablesHandle<'sr>,
        srunit:    &'sr SampleRateHandle<'sr>,
        time_unit: &'sr TimeUnitHandle<'sr>) -> Self 
    {
        Self {
            ringout:      Ringout::blocks(1000),
            params:       PhaserParam::new_runtime(), 
            mix:          Align16(LipolPs::new_with_blocksize(BLOCK_SIZE)),
            l:            Align16(Self::new_block()),
            r:            Align16(Self::new_block()),
            feedback:     LiPol::<f32>::new(BLOCK_SIZE * SLOWRATE),
            d_l:          Default::default(),
            d_r:          Default::default(),
            biquad:       Self::new_biquads(tuner,tables,srunit),
            lfophase:     0.0,
            bi:           0,
            tables:       tables.clone(),
            time_unit:    time_unit.clone(),
        }
    }
    #[inline] pub fn new_block() -> A1d::<f32> {
        A1d::<f32>::zeros(BLOCK_SIZE)
    }
    #[inline] pub fn new_biquads(
        tuner:     &'sr       TunerHandle<'sr>,
        tables:    &'sr       TablesHandle<'sr>,
        srunit:    &'sr       SampleRateHandle<'sr>,
    ) -> A1d::<BiquadFilter<'sr>> {
        A1d::<BiquadFilter>::from_elem(8, BiquadFilter::new(tuner,tables,srunit))
    }
}