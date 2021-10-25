ix!();

use crate::{
    Eq3Band,
    Eq3BandParam,
};

impl Init for Eq3Band<'sr> {

    fn init(&mut self) {

        let gain_override = self.instantize_unity_gain();

        self.update_all_bands(gain_override);

        self.coeff_instantize_all_bands();

        self.band1.suspend();
        self.band2.suspend();
        self.band3.suspend();

        self.block_increment = 0;
    }
}

impl Eq3Band<'sr> {

    pub fn new(
        tuner:  &'sr  TunerHandle<'sr>,
        tables: &'sr TablesHandle<'sr>,
        srunit: &'sr SampleRateHandle<'sr>,
    ) -> Self {

        let mut x = Self {
            tables:  tables.clone(),
            gain:    Align16(Default::default()),
            band1:   BiquadFilter::new(tuner,tables,srunit),
            band2:   BiquadFilter::new(tuner,tables,srunit),
            band3:   BiquadFilter::new(tuner,tables,srunit),
            ringout: Ringout::blocks(500),
            params:  Eq3BandParam::new_runtime(),
            block_increment: 0,
        };
        let srbs = (BLOCK_SIZE * SLOWRATE) as i32;
        x.gain.set_blocksize(BLOCK_SIZE as i32);
        x.band1.set_blocksize(srbs); // does not matter ATM as tey're smoothed
        x.band2.set_blocksize(srbs);
        x.band3.set_blocksize(srbs);
        x
    }
}