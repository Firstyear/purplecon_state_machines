use microwave_common::MicrowaveOps;

struct Microwave {
    door_open: bool,
    // This is an excellent example of why you always use
    // positive langage in booleans, rather than negatives :)
    magnetron_disabled: bool,
    time_remain: usize,
}

impl MicrowaveOps for Microwave {
    fn new() -> Self {
        Microwave {
            door_open: false,
            magnetron_disabled: true,
            time_remain: 0,
        }
    }

    fn reset(&mut self) {
        self.door_open = false;
        self.magnetron_disabled = true;
        self.time_remain = 0;
    }

    fn tick(&mut self) {
        // bug 2 - was not disabling mtron when time went to 0 due to incorrect if stmt.
        if !self.magnetron_disabled {
            if self.time_remain > 0 {
                self.time_remain -= 1;
            }
            // The tick has decremented, what's our new time?
            if self.time_remain == 0 {
                self.magnetron_disabled = true;
            }
        }
    }

    fn magnetron_enabled(&self) -> bool {
        !self.magnetron_disabled
    }

    fn door_open(&self) -> bool {
        self.door_open
    }

    fn time_remain(&self) -> usize {
        self.time_remain
    }

    fn action_open_door(&mut self) {
        self.door_open = true;
        if !self.magnetron_disabled {
            self.magnetron_disabled = true
        }
    }

    fn action_close_door(&mut self) {
        self.door_open = false
    }

    fn action_set_time(&mut self, t: usize) {
        if self.magnetron_disabled {
            self.time_remain = t
        }
    }

    fn action_start(&mut self) {
        // Bug 1 - I legit forgot to check this condition when starting ...
        if self.door_open == true {
            return;
        }

        // bug 2 - I was adding time, but not disabling mtron, leading to this
        // refactor.
        if self.magnetron_disabled == false {
            // we are running
            self.time_remain += 30
        } else {
            // not running, so start
            self.magnetron_disabled = false;
            if self.time_remain == 0 {
                self.time_remain = 30;
            }
        }
    }

    fn action_stop(&mut self) {
        // if running we stop
        if !self.magnetron_disabled {
            self.magnetron_disabled = true;
        } else {
            // but if already stopped, we now clear the time.
            self.time_remain = 0;
        }
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
