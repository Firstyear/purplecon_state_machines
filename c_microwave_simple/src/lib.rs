use microwave_common::MicrowaveOps;

extern {
    fn new_microwave() -> &'static mut MicrowaveC;
    fn reset_microwave(m: &mut MicrowaveC);
    fn tick_microwave(m: &mut MicrowaveC);
    fn magnetron_enabled_microwave(m: &MicrowaveC) -> bool;
    fn door_open_microwave(m: &MicrowaveC) -> bool;
    fn time_remain_microwave(m: &MicrowaveC) -> usize;

    fn action_open_door_microwave(m: &mut MicrowaveC);
    fn action_close_door_microwave(m: &mut MicrowaveC);
    fn action_set_time_microwave(m: &mut MicrowaveC, t: usize);
    fn action_start_microwave(m: &mut MicrowaveC);
    fn action_stop_microwave(m: &mut MicrowaveC);
}

// Basically a typed void pointer.
#[allow(improper_ctypes)]
#[repr(C)]
struct MicrowaveC {}

struct Microwave {
    mwave: &'static mut MicrowaveC
}

impl MicrowaveOps for Microwave {
    fn new() -> Self {
        Microwave {
            mwave: unsafe { new_microwave() }
        }
    }

    fn reset(&mut self) {
        unsafe { reset_microwave(self.mwave) }
    }

    fn tick(&mut self) {
        unsafe { tick_microwave(self.mwave) };
    }

    fn magnetron_enabled(&self) -> bool {
        unsafe { magnetron_enabled_microwave(self.mwave) }
    }

    fn door_open(&self) -> bool {
        unsafe { door_open_microwave(self.mwave) }
    }

    fn time_remain(&self) -> usize {
        unsafe { time_remain_microwave(self.mwave) }
    }

    fn action_open_door(&mut self) {
        unsafe { action_open_door_microwave(self.mwave) };
    }

    fn action_close_door(&mut self) {
        unsafe { action_close_door_microwave(self.mwave) };
    }

    fn action_set_time(&mut self, t: usize) {
        unsafe { action_set_time_microwave(self.mwave, t) };
    }

    fn action_start(&mut self) {
        unsafe { action_start_microwave(self.mwave) };
    }

    fn action_stop(&mut self) {
        unsafe { action_stop_microwave(self.mwave) };
    }
}

#[cfg(test)]
mod tests {
    use crate::Microwave;
    use microwave_common::{MicrowaveOps, test_microwave};

    #[test]
    fn it_works() {
        let mut mw = Microwave::new();
        assert!(test_microwave(&mut mw));
    }
}
