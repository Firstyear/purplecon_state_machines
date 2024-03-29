use microwave_common::MicrowaveOps;

/*
 * DIY Microwave! Just fill in the blanks, then run "cargo test" from this directory.
 *
 * Good Luck!
 */

struct Microwave {}

impl MicrowaveOps for Microwave {
    fn new() -> Self {
        // Hint: When you get a microwave and unbox it, the door is closed, and no
        // power, so the time is probably 0 :)
        Microwave {}
    }

    fn reset(&mut self) {
    }

    fn tick(&mut self) {
    }

    fn magnetron_enabled(&self) -> bool {
        false
    }

    fn door_open(&self) -> bool {
        false
    }

    fn time_remain(&self) -> usize {
        0
    }

    fn action_open_door(&mut self) {
    }

    fn action_close_door(&mut self) {
    }

    fn action_set_time(&mut self, t: usize) {
    }

    fn action_start(&mut self) {
    }

    fn action_stop(&mut self) {
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
