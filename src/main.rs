extern crate tokio_timer;

use std::thread;
use std::time::Duration;
use tokio_timer::Timer;

fn main() {
		let _timer = Timer::default();
		thread::sleep(Duration::from_millis(2000));
}
