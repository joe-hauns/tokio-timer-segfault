extern crate tokio_timer;
use tokio_timer::*;
use tokio_timer as builder;
use tokio_timer::wheel;

use std::time::Instant;
use builder::Builder;
use wheel::{Wheel, Slot, Token, EMPTY};

fn main() {
    let wheel = Wheel::new(&Builder {
        tick_duration: None,
        num_slots: None,
        initial_capacity: None,
        max_capacity: None,
        max_timeout: None,
        channel_capacity: None,
        thread_name: None,
    });
    wheel.next_timeout();
}


