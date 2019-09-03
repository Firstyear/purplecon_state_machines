✨STATE MACHINES✨
==================

State machines are a way of modeling event driven systems to allow reasoning and analysis
of their behaviours and properties.

🚨In Progress🚨
---------------

These resources and the associated presentation will be presented at PurpleCon NZ on
October 16th. Until then, this is in progress.

How to use these resources?
---------------------------

Read and follow the README - throughout we will reference the code in the various subdirectories.


These programs are all writing in Rust or C. It's not necessary to run the tests to value from this - reading the code is sufficient.

If you want to run the tests however, all examples are managed by cargo (even the C examples).
Please follow your platforms guide for "rustup" to setup a compatible environment. Once you have
Rust working, you can run the tests with:

    cargo test

The Microwave
-------------

Microwaves are a state machine, where importantly, the wrong action causes you to microwave
the poor, innocent user. We don't want that! As a result, we want to model the microwaves actions
based on what you *expect*. Let's list a few.

* If you press start and the door is open, it won't activate.
* If it's running and you open the door, it stops.
* If you press start and it's running, you add 30 seconds.
* The timer counts down, and then stops the microwave (overcooked food is also the enemy).
* And many more ...

To really get a hold of this, we need to model this state machine. What are our states?

* OPEN_NOTIME
* OPEN_TIME
* CLOSED_NOTIME_NOMTRON
* CLOSED_TIME_NOMTRON
* CLOSED_TIME_MTRON

What are our events or transitions?

* open the door
* close the door
* set time
* press stop
* press start
* one second elapses

Now we need to examine each state and what valid transitions occur. We can arrange this in a table, or draw a diagram.

    |            | OPEN_NOTIME           | OPEN_TIME           | CLOSED_NOTIME_NOMTRON | CLOSED_TIME_NOMTRON   | CLOSED_TIME_MTRON   |
    | open door  | OPEN_NOTIME           | OPEN_TIME           | OPEN_NOTIME           | OPEN_TIME             | OPEN_TIME           |
    | close door | CLOSED_NOTIME_NOMTRON | CLOSED_TIME_NOMTRON | CLOSED_NOTIME_NOMTRON | CLOSED_TIME_NOMTRON   | CLOSED_TIME_MTRON   |
    | set time   | OPEN_TIME             | OPEN_TIME           | CLOSED_TIME_NOMTRON   | CLOSED_TIME_NOMTRON   | CLOSED_TIME_MTRON   |
    | stop       | OPEN_NOTIME           | OPEN_NOTIME         | CLOSED_NOTIME_NOMTRON | CLOSED_NOTIME_NOMTRON | CLOSED_TIME_NOMTRON |
    | start      | OPEN_NOTIME           | OPEN_TIME           | CLOSED_TIME_MTRON     | CLOSED_TIME_MTRON     | CLOSED_TIME_MTRON   |
    | one second | OPEN_NOTIME           | OPEN_TIME           | CLOSED_NOTIME_NOMTRON | CLOSED_TIME_NOMTRON   | CLOSED_TIME_MTRON OR CLOSED_NOTIME_NOMTRON |

The only complexity here is second counting with the CLOSED_TIME_MTRON state - if the time reaches zero we move to "notime" otherwise, we stay in CLOSED_TIME_MTRON.

To make it a bit clearer, lets blank the rows where the same state is remained in.


    |            | OPEN_NOTIME           | OPEN_TIME           | CLOSED_NOTIME_NOMTRON | CLOSED_TIME_NOMTRON   | CLOSED_TIME_MTRON   |
    | open door  | -                     | -                   | OPEN_NOTIME           | OPEN_TIME             | OPEN_TIME           |
    | close door | CLOSED_NOTIME_NOMTRON | CLOSED_TIME_NOMTRON | -                     | -                     | -                   |
    | set time   | OPEN_TIME             | -                   | CLOSED_TIME_NOMTRON   | -                     | -                   |
    | stop       | -                     | OPEN_NOTIME         | -                     | CLOSED_NOTIME_NOMTRON | CLOSED_TIME_NOMTRON |
    | start      | -                     | -                   | CLOSED_TIME_MTRON     | CLOSED_TIME_MTRON     | -                   |
    | one second | -                     | -                   | -                     | -                     | CLOSED_TIME_MTRON OR CLOSED_NOTIME_NOMTRON |

That's a bit easier. Now we can see where the events will *change* our state rather that leaving us in the same state.

From this we can draw a pretty picture:

<p align="center">
    <img src="https://raw.githubusercontent.com/Firstyear/purplecon_state_machines/master/microwave.png" width="80%" height="auto" />
</p>

What's important is every input, at every state is *well defined* with a known safe or expected behaviour. This not only helps us to design the software
but to design a series of test cases that will exercise or stress this model to ensure it is correct.

Testing the Microwave
---------------------

With the above states above, we can now perform tests by checking the state of:

* The door
* The magnetron
* The time remaining

And by manipulating the state by the 6 transitions. You can find the test harness in `microwave_common/` to show how you can exhaustively test the state machine above.

DIY?
----

Feeling inspired? Ready to give it a go?

Just fill in the blanks in "rust_microwave_diy" to practice your new state machine designing skills!



