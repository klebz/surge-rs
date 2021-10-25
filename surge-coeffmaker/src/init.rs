ix!();

use crate::FilterCoefficientMaker;

impl FilterCoefficientMaker<'sr> {

    pub fn new( 
        tuner:  TunerHandle<'sr>,
        tables: TablesHandle<'sr>,
        srunit: SampleRateHandle<'sr>,
    ) -> Self {

        let mut x = Self {
            coeff:      A1d::<f32>::zeros(N_COEFFMAKER_COEFFS),
            dcoeff:     A1d::<f32>::zeros(N_COEFFMAKER_COEFFS),
            tcoeff:     A1d::<f32>::zeros(N_COEFFMAKER_COEFFS),
            first_run:  true,
            tuner:      tuner.clone(),
            tables:     tables.clone(),
            srunit:     srunit.clone(),
        };
        x.reset();
        x
    }
}

impl Reset for FilterCoefficientMaker<'sr> 
{
    fn reset(&mut self) 
    {
        self.first_run = true;
        self.coeff.fill(0.0);
        self.dcoeff.fill(0.0);
        self.tcoeff.fill(0.0);
    }
}