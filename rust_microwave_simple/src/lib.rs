use microwave_common::MicrowaveOps;

#[derive(Clone, Copy)]
enum MicrowaveState {
    OpenNoTime,
    OpenTime(usize),
    ClosedNoTimeNoMtron,
    ClosedTimeNoMtron(usize),
    ClosedTimeMtron(usize),
}

struct Microwave {
    state: MicrowaveState
}

impl MicrowaveOps for Microwave {
    fn new() -> Self {
        Microwave {
            state: MicrowaveState::ClosedNoTimeNoMtron,
        }
    }

    fn reset(&mut self) {
        self.state = MicrowaveState::ClosedNoTimeNoMtron;
    }

    fn tick(&mut self) {
        self.state = match self.state {
            MicrowaveState::ClosedTimeMtron(mut time) => {
                // prevent underflow
                if time > 0 {
                    time -= 1;
                }
                if time == 0 {
                    MicrowaveState::ClosedNoTimeNoMtron
                } else {
                    MicrowaveState::ClosedTimeMtron(time)
                }
            }
            s => s
        };
    }

    fn magnetron_enabled(&self) -> bool {
        match self.state {
            MicrowaveState::ClosedTimeMtron(_) => true,
            _ => false,
        }
    }

    fn door_open(&self) -> bool {
        match self.state {
            MicrowaveState::OpenNoTime | MicrowaveState::OpenTime(_) => true,
            _ => false,
        }
    }

    fn time_remain(&self) -> usize {
        match self.state {
            MicrowaveState::OpenTime(t) | MicrowaveState::ClosedTimeNoMtron(t) | MicrowaveState::ClosedTimeMtron(t) => t,
            _ => 0,
        }
    }

    fn action_open_door(&mut self) {
        self.state = match self.state {
            MicrowaveState::ClosedTimeNoMtron(t) => MicrowaveState::OpenTime(t),
            MicrowaveState::ClosedTimeMtron(t) => MicrowaveState::OpenTime(t),
            MicrowaveState::ClosedNoTimeNoMtron => MicrowaveState::OpenNoTime,
            s => s,
        }
    }

    fn action_close_door(&mut self) {
        self.state = match self.state {
            MicrowaveState::OpenTime(t) => MicrowaveState::ClosedTimeNoMtron(t),
            MicrowaveState::OpenNoTime => MicrowaveState::ClosedNoTimeNoMtron,
            s => s,
        }
    }

    fn action_set_time(&mut self, t: usize) {
        self.state = match self.state {
            MicrowaveState::ClosedTimeNoMtron(_) => MicrowaveState::ClosedTimeNoMtron(t),
            MicrowaveState::ClosedNoTimeNoMtron => MicrowaveState::ClosedTimeNoMtron(t),
            MicrowaveState::OpenNoTime => MicrowaveState::OpenTime(t),
            MicrowaveState::OpenTime(_) => MicrowaveState::OpenTime(t),
            s => s,
        }
    }

    fn action_start(&mut self) {
        self.state = match self.state {
            MicrowaveState::ClosedNoTimeNoMtron => MicrowaveState::ClosedTimeMtron(30),
            MicrowaveState::ClosedTimeNoMtron(t) => MicrowaveState::ClosedTimeMtron(t),
            MicrowaveState::ClosedTimeMtron(t) => MicrowaveState::ClosedTimeMtron(t + 30),
            s => s,
        }
    }

    fn action_stop(&mut self) {
        self.state = match self.state {
            MicrowaveState::ClosedTimeMtron(t) => MicrowaveState::ClosedTimeNoMtron(t),
            MicrowaveState::ClosedTimeNoMtron(_) => MicrowaveState::ClosedNoTimeNoMtron,
            MicrowaveState::OpenTime(_) => MicrowaveState::OpenNoTime,
            s => s
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
