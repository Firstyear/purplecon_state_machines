
#[derive(Debug)]
struct OpenNoTime;
#[derive(Debug)]
struct OpenTime { t: usize }
#[derive(Debug)]
struct ClosedNoTimeNoMtron;
#[derive(Debug)]
struct ClosedTimeNoMtron { t: usize }
#[derive(Debug)]
struct ClosedTimeMtron { t: usize }

#[derive(Debug)]
struct Microwave<STATE> {
    state: STATE
}

impl Microwave<OpenNoTime> {
    fn magnetron_enabled(&self) -> bool {
        false
    }

    fn door_open(&self) -> bool {
        true
    }

    fn time_remain(&self) -> usize {
        0
    }

    fn action_close_door(self) -> Microwave<ClosedNoTimeNoMtron> {
        Microwave {
            state: ClosedNoTimeNoMtron
        }
    }

    fn action_set_time(&mut self, t: usize) -> Microwave<OpenTime> {
        Microwave {
            state: OpenTime { t: t }
        }
    }
}

impl Microwave<OpenTime> {
    fn magnetron_enabled(&self) -> bool {
        false
    }

    fn door_open(&self) -> bool {
        true
    }

    fn time_remain(&self) -> usize {
        self.state.t
    }

    fn action_close_door(self) -> Microwave<ClosedTimeNoMtron> {
        Microwave {
            state: ClosedTimeNoMtron { t: self.state.t }
        }
    }

    fn action_set_time(self, t: usize) -> Self {
        Microwave {
            state: OpenTime { t: t }
        }
    }

    fn action_stop(self) -> Microwave<OpenNoTime> {
        Microwave {
            state: OpenNoTime
        }
    }
}

impl Microwave<ClosedNoTimeNoMtron> {
    fn new() -> Self {
        Microwave {
            state: ClosedNoTimeNoMtron
        }
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

    fn action_open_door(self) -> Microwave<OpenNoTime> {
        Microwave {
            state: OpenNoTime,
        }
    }

    fn action_set_time(self, t: usize) -> Microwave<ClosedTimeNoMtron> {
        Microwave {
            state: ClosedTimeNoMtron { t: t }
        }
    }

    fn action_start(self) -> Microwave<ClosedTimeMtron> {
        Microwave {
            state: ClosedTimeMtron { t: 30 }
        }
    }
}

impl Microwave<ClosedTimeNoMtron> {
    fn magnetron_enabled(&self) -> bool {
        false
    }

    fn door_open(&self) -> bool {
        false
    }

    fn time_remain(&self) -> usize {
        self.state.t
    }

    fn action_open_door(self) -> Microwave<OpenTime> {
        Microwave {
            state: OpenTime { t: self.state.t },
        }
    }

    fn action_set_time(self, t: usize) -> Microwave<ClosedTimeNoMtron> {
        Microwave {
            state: ClosedTimeNoMtron { t: t }
        }
    }

    fn action_start(self) -> Microwave<ClosedTimeMtron> {
        Microwave {
            state: ClosedTimeMtron { t: self.state.t }
        }
    }

    fn action_stop(self) -> Microwave<ClosedNoTimeNoMtron> {
        Microwave {
            state: ClosedNoTimeNoMtron
        }
    }
}

impl Microwave<ClosedTimeMtron> {
    fn magnetron_enabled(&self) -> bool {
        true
    }

    fn door_open(&self) -> bool {
        false
    }

    fn time_remain(&self) -> usize {
        self.state.t
    }

    fn tick(self) -> Result<Microwave<ClosedTimeMtron>, Microwave<ClosedNoTimeNoMtron>> {
        if self.state.t == 1 {
            Err(Microwave {
                state: ClosedNoTimeNoMtron
            })
        } else {
            Ok(Microwave {
                state: ClosedTimeMtron { t: self.state.t - 1 }
            })
        }
    }

    fn action_open_door(self) -> Microwave<OpenTime> {
        Microwave {
            state: OpenTime { t: self.state.t },
        }
    }

    fn action_start(self) -> Microwave<ClosedTimeMtron> {
        Microwave {
            state: ClosedTimeMtron { t: self.state.t + 30 }
        }
    }

    fn action_stop(self) -> Microwave<ClosedTimeNoMtron> {
        Microwave {
            state: ClosedTimeNoMtron { t: self.state.t }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Microwave;
    use crate::{ OpenNoTime, OpenTime, ClosedNoTimeNoMtron, ClosedTimeNoMtron, ClosedTimeMtron };

    // Due to the fact these are compiled, not runtime, we have to take
    // a different approach to the tests!

    macro_rules! assert_mw {
        (
            $mw:expr,
            $door:expr,
            $mtron:expr,
            $time:expr
        ) => {{
            // Assert we never have an unsafe combo:
            if $mw.magnetron_enabled() {
                assert!($mw.door_open() == false)
            }
            assert!($mw.door_open() == $door);
            assert!($mw.magnetron_enabled() == $mtron);
            assert!($mw.time_remain() == $time);
        }};
    }

    #[test]
    fn test_1() {
        let mut mw: Microwave<ClosedNoTimeNoMtron> = Microwave::new();
        assert_mw!(mw, false, false, 0);
        let mut mw: Microwave<OpenNoTime> = mw.action_open_door();
        assert_mw!(mw, true, false, 0);
        // It's literally impossible to compile the .start() check
        // here. Uncomment any try.
        // let mut mw: Microwave<ClosedTimeMtron> = mw.action_start();
        let mut mw: Microwave<OpenTime> = mw.action_set_time(20);
        assert_mw!(mw, true, false, 20);
        let mut mw: Microwave<OpenTime> = mw.action_set_time(30);
        assert_mw!(mw, true, false, 30);
        let mut mw: Microwave<ClosedTimeNoMtron> = mw.action_close_door();
        assert_mw!(mw, false, false, 30);
        let mut mw: Microwave<OpenTime> = mw.action_open_door();
        assert_mw!(mw, true, false, 30);
        let mut mw: Microwave<OpenNoTime> = mw.action_stop();
        assert_mw!(mw, true, false, 0);
    }

    #[test]
    fn test_2() {
        // Test 2 literally can not compile with the type based states!
    }

    #[test]
    fn test_3() {
        let mut mw: Microwave<ClosedNoTimeNoMtron> = Microwave::new();
        assert_mw!(mw, false, false, 0);
        let mut mw: Microwave<ClosedTimeNoMtron> = mw.action_set_time(2);
        assert_mw!(mw, false, false, 2);
        let mut mw: Microwave<ClosedTimeMtron> = mw.action_start();
        assert_mw!(mw, false, true, 2);
        // Now we can use result combinators on the ok/err and get the state
        // back.
        let mut mw: Microwave<ClosedTimeMtron> = mw.tick().unwrap();
        assert_mw!(mw, false, true, 1);
        let mut mw: Microwave<ClosedNoTimeNoMtron> = mw.tick().unwrap_err();
        assert_mw!(mw, false, false, 0);
    }

    #[test]
    fn test_4() {
        let mut mw: Microwave<ClosedNoTimeNoMtron> = Microwave::new();
        assert_mw!(mw, false, false, 0);
        let mut mw: Microwave<ClosedTimeMtron> = mw.action_start();
        assert_mw!(mw, false, true, 30);
        let mut mw: Microwave<ClosedTimeMtron> = mw.action_start();
        assert_mw!(mw, false, true, 60);
        let mut mw: Microwave<ClosedTimeMtron> = mw.tick().unwrap();
        assert_mw!(mw, false, true, 59);
        let mut mw: Microwave<OpenTime> = mw.action_open_door();
        assert_mw!(mw, true, false, 59);
        let mut mw: Microwave<ClosedTimeNoMtron> = mw.action_close_door();
        assert_mw!(mw, false, false, 59);
        let mut mw: Microwave<ClosedTimeMtron> = mw.action_start();
        assert_mw!(mw, false, true, 59);
        let mut mw: Microwave<ClosedTimeMtron> = mw.tick().unwrap();
        assert_mw!(mw, false, true, 58);
        let mut mw: Microwave<ClosedTimeNoMtron> = mw.action_stop();
        assert_mw!(mw, false, false, 58);
        let mut mw: Microwave<ClosedNoTimeNoMtron> = mw.action_stop();
        assert_mw!(mw, false, false, 0);
    }

    #[test]
    fn test_5() {
        let mut mw: Microwave<ClosedNoTimeNoMtron> = Microwave::new();
        assert_mw!(mw, false, false, 0);
        let mut mw: Microwave<ClosedTimeMtron> = mw.action_start();
        assert_mw!(mw, false, true, 30);
        let mut mw: Microwave<OpenTime> = mw.action_open_door();
        assert_mw!(mw, true, false, 30);
        let mut mw: Microwave<OpenTime> = mw.action_set_time(25);
        assert_mw!(mw, true, false, 25);
    }

    #[test]
    fn test_6() {
        let mut mw: Microwave<ClosedNoTimeNoMtron> = Microwave::new();
        assert_mw!(mw, false, false, 0);
        let mut mw: Microwave<OpenNoTime> = mw.action_open_door();
        assert_mw!(mw, true, false, 0);
        let mut mw: Microwave<OpenTime> = mw.action_set_time(25);
        assert_mw!(mw, true, false, 25);
        let mut mw: Microwave<ClosedTimeNoMtron> = mw.action_close_door();
        assert_mw!(mw, false, false, 25);
        let mut mw: Microwave<ClosedTimeNoMtron> = mw.action_set_time(35);
        assert_mw!(mw, false, false, 35);
        let mut mw: Microwave<OpenTime> = mw.action_open_door();
        assert_mw!(mw, true, false, 35);
    }
}
