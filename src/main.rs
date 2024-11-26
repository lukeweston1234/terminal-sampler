use std::time::Duration;

use crossbeam::{channel::tick, select};

enum ClockMessage {
    Tick,
    Stopped,
}

enum ClockCommand {
    Start,
    ChangeBPM(f32),
    Stop
}

struct Clock {
    bpm: f64,
}
impl Clock {
    pub fn new(bpm: f64) -> Self {
        Self {
            bpm
        }
    }
}

pub fn run_clock(mut clock: Clock){
    let ticker = tick(Duration::from_secs_f64(60.0 / (clock.bpm * 24.0)));

    std::thread::spawn(move || {
        loop {
            select! {
                recv(ticker) -> _ => println!("tick!")
            }
        }
        
    });
}



fn main() {
    let clock = Clock::new(120.0);

    run_clock(clock);

    std::thread::sleep(std::time::Duration::from_secs(1));
}
