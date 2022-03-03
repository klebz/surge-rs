ix!();

use crate::SurgeSynthesizer;

impl SurgeSynthesizer<'plugin_layer> {

    pub fn all_notes_off(&mut self) {

        for i in 0..16 {
            self.midi_unit.set_hold(i,false);

            for k in 0..128 {
                self.midi_unit.set_keystate(i,k,0);
                self.midi_unit.set_lastdetune(i,k,0);
            }
        }

        self.active_patch.scene[0].all_notes_off();
        self.active_patch.scene[1].all_notes_off();

        self.active_patch.scene[0].hold_pedal_unit.clear_holdbuffer();
        self.active_patch.scene[1].hold_pedal_unit.clear_holdbuffer();

        self.active_patch.scene[0].halfband.reset();
        self.active_patch.scene[1].halfband.reset();

        self.halfband_in.reset();

        self.active_patch.scene[0].highpass.suspend();
        self.active_patch.scene[1].highpass.suspend();

        for i in 0..8 {
            self.fx_unit.fx[i].suspend()
        }

    }
}

//TODO eliminate this
impl Drop for SurgeSynthesizer<'plugin_layer>  {

    fn drop(&mut self) {

        self.all_notes_off();

        for state in self.active_patch.scene[0].fbq.state.iter_mut() { state.init(); }
        for state in self.active_patch.scene[1].fbq.state.iter_mut() { state.init(); }

        let patch = &mut self.active_patch;

        for i in 0_usize..N_CUSTOMCONTROLLERS {
            let ms = ModSource::ctrl(i);
            patch.scene[0].modsources[ms] = None;
        }
    }
}
