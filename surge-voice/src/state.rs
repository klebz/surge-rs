crate::ix!();

#[derive(Debug)]
pub struct SurgeVoiceState {
    pub gate:                bool,
    pub keep_playing:        bool,
    pub uberrelease:         bool,
    pub pitch:               f64,
    pub fvel:                f64,
    pub pkey:                f64,
    pub detune:              f64,
    pub freleasevel:         f64,
    pub keystate:            *mut MidiKeyState,
    pub main_channel_state:  *mut MidiChannelState,
    pub voice_channel_state: *mut MidiChannelState,
    pub key:                 i32,
    pub velocity:            i32,
    pub channel:             i32,
    pub releasevelocity:     i32,
    pub portasrc_key:        f64,
    pub portaphase:          f64,
}

impl SurgeVoiceState {

    pub fn default_pitch() -> f64 { todo!(); }
    pub fn default_pkey()  -> f64 { todo!(); }

    pub fn get_pitch(&self) -> f32 
    {
        let key      = self.key as f32;
        let voice_pb = unsafe { (*self.voice_channel_state).pitchbend_in_semitones.0 };
        let main_pb  = unsafe { (*self.main_channel_state).pitchbend_in_semitones.0 };
        let detune   = self.detune as f32;

        /*
         | For this commented out section, see
         | the comment on MPE global pitch bend
         | in
         | 
         | SurgeSynthesizer::pitchBend
         |
         */
        key + voice_pb + detune + main_pb
    }

    pub fn set_portasrc_key(&mut self, 
        polymode:       PolyMode, 
        portamento_min: bool, 
        last_key:       i32, 
        pitch:          f64) 
    {
        let sel_polymode = polymode == PolyMode::MonoSingleTriggerFingeredPortamento;

        self.portasrc_key = match (sel_polymode, portamento_min) {
            (true, true) => pitch,
            _            => last_key as f64,
        };
    }
}
