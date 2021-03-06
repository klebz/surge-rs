crate::ix!();

/**
  | This contains an adaptation of the filter
  | found at https://ccrma.stanford.edu/~jatin/ComplexNonlinearities/NLBiquad.html
  | with coefficient calculation from
  | https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html
  | 
  | a lot of code here is duplicated from
  | 
  | NonlinearFeedback.cpp, perhaps in
  | future they could be merged, but for
  | the time being they're separate and
  | nothing is shared.
  |
  */
#[derive(Derivative)] #[derivative(Debug)]
pub struct NonlinearStatesFilter {
    pub tuner:  TunerHandle,
    pub srunit: SampleRateHandle,
    pub ty:     NLSFType,
}

impl NonlinearStatesFilter {
    pub fn clamped_frequency(&self, pitch: f32) -> f32
    {
        let freq = self.tuner.n2p::<f32,true>(pitch + 69.0) * (MIDI_0_FREQ as f32);
        limit_range(freq, 5.0, self.srunit.samplerate_os() * 0.3)
    }
}
