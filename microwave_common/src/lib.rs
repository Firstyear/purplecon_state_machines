
pub trait MicrowaveOps {
    fn new() -> Self;
    fn reset(&mut self);

    fn tick(&mut self);

    fn magnetron_enabled(&self) -> bool;
    fn door_open(&self) -> bool;
    fn time_remain(&self) -> usize;

    fn action_open_door(&mut self);
    fn action_close_door(&mut self);
    fn action_set_time(&mut self, t: usize);
    fn action_start(&mut self);
    fn action_stop(&mut self);
}

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

pub fn test_microwave<T: MicrowaveOps>(mw: &mut T) -> bool {
    // Test 1 - check that from both door-open states, pressing start will never activate the
    // magnetron.
    println!("Test 1");
    mw.reset();
    // CLOSED_NOTIME_NOMTRON -> OPEN_NOTIME
    mw.action_open_door();
    assert_mw!(mw, true, false, 0);
    // OPEN_NOTIME -> CLOSED_NOTIME_NOMTRON
    mw.action_close_door();
    assert_mw!(mw, false, false, 0);
    // CLOSED_NOTIME_NOMTRON -> OPEN_NOTIME
    mw.action_open_door();
    assert_mw!(mw, true, false, 0);

    // OPEN_NOTIME -> X
    mw.action_start();
    assert_mw!(mw, true, false, 0);
    // OPEN_NOTIME -> X
    mw.action_stop();
    assert_mw!(mw, true, false, 0);

    // OPEN_NOTIME -> OPEN_TIME
    mw.action_set_time(20);
    assert_mw!(mw, true, false, 20);
    // OPEN_TIME -> OPEN_TIME
    mw.action_set_time(30);
    assert_mw!(mw, true, false, 30);
    // OPEN_TIME -> X
    mw.action_open_door();
    assert_mw!(mw, true, false, 30);
    // OPEN_TIME -> X
    mw.action_start();
    assert_mw!(mw, true, false, 30);

    // OPEN_TIME -> CLOSED_NOTIME_NOMTRON
    mw.action_close_door();
    assert_mw!(mw, false, false, 30);
    // CLOSED_NOTIME_NOMTRON -> OPEN_TIME
    mw.action_open_door();
    assert_mw!(mw, true, false, 30);

    // OPEN_TIME -> OPEN_NOTIME
    mw.action_stop();
    assert_mw!(mw, true, false, 0);


    // Test 2 - check that when the magnetron is disabled, time ticks have no effect.
    println!("Test 1");
    mw.reset();
    mw.action_open_door();
    assert_mw!(mw, true, false, 0);
    // OPEN_NOTIME -> X
    mw.tick();
    assert_mw!(mw, true, false, 0);

    mw.reset();
    mw.action_open_door();
    assert_mw!(mw, true, false, 0);
    mw.action_set_time(30);
    assert_mw!(mw, true, false, 30);
    // OPEN_TIME -> X
    mw.tick();
    assert_mw!(mw, true, false, 30);

    mw.reset();
    mw.action_close_door();
    // CLOSED_NOTIME_NOMTRON -> X
    mw.tick();
    assert_mw!(mw, false, false, 0);

    mw.reset();
    mw.action_close_door();
    assert_mw!(mw, false, false, 0);
    mw.action_set_time(30);
    assert_mw!(mw, false, false, 30);
    // CLOSED_TIME_NOMTRON -> X
    mw.tick();
    assert_mw!(mw, false, false, 30);

    // This set's the time *before* we close the door to exercise open_time
    // to close_time_nomtron
    mw.reset();
    mw.action_open_door();
    mw.action_set_time(30);
    assert_mw!(mw, true, false, 30);
    mw.action_close_door();
    assert_mw!(mw, false, false, 30);
    // CLOSED_TIME_NOMTRON -> X
    mw.tick();
    assert_mw!(mw, false, false, 30);

    // Test 3 - time to heat some food my friens.
    // Check the timer decrements and ends as expected.
    println!("Test 3");
    mw.reset();
    // CLOSED_NOTIME_NOMTRON -> X
    mw.action_close_door();
    // CLOSED_NOTIME_NOMTRON -> CLOSED_TIME_NOMTRON
    mw.action_set_time(2);
    assert_mw!(mw, false, false, 2);
    // CLOSED_TIME_NOMTRON -> CLOSED_TIME_MTRON
    mw.action_start();
    assert_mw!(mw, false, true, 2);
    // CLOSED_TIME_MTRON -> X
    mw.tick();
    assert_mw!(mw, false, true, 1);
    // CLOSED_TIME_MTRON -> CLOSED_NOTIME_NOMTRON
    mw.tick();
    assert_mw!(mw, false, false, 0);

    // Test 4 - Check that pressing start with no time advances to 30. We check some other interactions
    // of pressing the start with time also.
    println!("Test 4");
    mw.reset();
    // CLOSED_NOTIME_NOMTRON
    mw.action_close_door();
    assert_mw!(mw, false, false, 0);
    // CLOSED_NOTIME_NOMTRON -> CLOSED_TIME_MTRON
    mw.action_start();
    assert_mw!(mw, false, true, 30);
    // CLOSED_TIME_MTRON -> X
    mw.action_start();
    assert_mw!(mw, false, true, 60);
    mw.tick();
    assert_mw!(mw, false, true, 59);
    // CLOSED_TIME_MTRON -> OPEN_TIME
    mw.action_open_door();
    assert_mw!(mw, true, false, 59);
    // OPEN_TIME -> CLOSED_TIME_NOMTRON
    mw.action_close_door();
    mw.action_start();
    assert_mw!(mw, false, true, 59);
    mw.tick();
    assert_mw!(mw, false, true, 58);
    // CLOSED_TIME_MTRON -> CLOSED_TIME_NOMTRON
    mw.action_stop();
    assert_mw!(mw, false, false, 58);
    // CLOSED_TIME_NOMTRON -> X
    mw.tick();
    assert_mw!(mw, false, false, 58);
    // CLOSED_TIME_NOMTRON -> CLOSED_NOTIME_NOMTRON
    mw.action_stop();
    assert_mw!(mw, false, false, 0);

    // Test 5 - check that changes to time while running have no effect.
    println!("Test 5");
    mw.reset();
    mw.action_close_door();
    // CLOSED_TIME_MTRON
    mw.action_start();
    assert_mw!(mw, false, true, 30);
    // CLOSED_TIME_MTRON -> X
    mw.action_set_time(25);
    assert_mw!(mw, false, true, 30);
    // CLOSED_TIME_MTRON -> X
    mw.action_set_time(45);
    assert_mw!(mw, false, true, 30);
    // CLOSED_TIME_MTRON -> OPEN_TIME
    mw.action_open_door();
    assert_mw!(mw, true, false, 30);
    // OPEN_TIME -> OPEN_TIME
    mw.action_set_time(45);
    assert_mw!(mw, true, false, 45);

    // Test 6 - Time to test weird stuff.
    // This tests all the edge cases that we normally don't think of like double closes
    // or time changes in certain states.
    println!("Test 6");
    println!("Well done to make it to this point! 🎉");
    mw.reset();

    // CLOSED_NOTIME_NOMTRON -> OPEN_NOTIME
    mw.action_open_door();
    assert_mw!(mw, true, false, 0);
    // OPEN_NOTIME -> X
    mw.action_open_door();
    assert_mw!(mw, true, false, 0);

    // OPEN_NOTIME -> OPEN_TIME
    mw.action_set_time(24);
    assert_mw!(mw, true, false, 24);
    // OPEN_TIME -> X
    mw.action_open_door();
    assert_mw!(mw, true, false, 24);

    // OPEN_TIME -> CLOSED_TIME_NOMTRON
    mw.action_close_door();
    assert_mw!(mw, false, false, 24);
    // CLOSED_TIME_NOMTRON -> X
    mw.action_set_time(389);
    assert_mw!(mw, false, false, 389);
    // CLOSED_TIME_NOMTRON -> X
    mw.action_close_door();
    assert_mw!(mw, false, false, 389);
    // CLOSED_TIME_NOMTRON -> OPEN_TIME
    mw.action_open_door();
    assert_mw!(mw, true, false, 389);

    mw.reset();
    // CLOSED_NOTIME_NOMTRON -> X
    mw.action_close_door();
    assert_mw!(mw, false, false, 0);
    // CLOSED_NOTIME_NOMTRON -> X
    mw.action_close_door();
    assert_mw!(mw, false, false, 0);
    // CLOSED_NOTIME_NOMTRON -> X
    mw.action_stop();
    assert_mw!(mw, false, false, 0);
    // CLOSED_NOTIME_NOMTRON -> OPEN_NOTIME
    mw.action_open_door();
    assert_mw!(mw, true, false, 0);

    // OPEN_NOTIME -> CLOSED_NOTIME_NOMTRON -> CLOSED_TIME_MTRON
    mw.action_close_door();
    assert_mw!(mw, false, false, 0);
    mw.action_start();
    assert_mw!(mw, false, true, 30);
    // CLOSED_TIME_MTRON -> X
    mw.action_close_door();
    assert_mw!(mw, false, true, 30);

    println!("✨ Your implementation passes! ✨");
    true
}

