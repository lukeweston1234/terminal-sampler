mod clock;

use crossbeam::channel::bounded;

use crate::clock::{build_clock, run_clock};

fn main() {
    let (clock_controller, clock) = build_clock();

    let (tx, rx) = bounded(10);

    clock_controller.add_sender(tx);

    run_clock(clock);

    clock_controller.start();

    loop {
        if let Ok(msg) = rx.try_recv(){
            println!("msg!, {:?}", msg);
        }
    }
}
