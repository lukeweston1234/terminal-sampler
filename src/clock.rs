use std::time::{Duration, Instant};

use crossbeam::{channel::{bounded, tick, Receiver, Sender}, select};

#[derive(Clone, Debug)]
pub enum ClockMessage {
    Start,
    Tick(Instant),
    Stopped,
}

#[derive(Clone, Debug)]
pub enum ClockCommand {
    Start,
    ChangeBPM(f64),
    AddSender(Sender<ClockMessage>),
    Stop
}

#[derive(Clone)]
pub struct ClockController {
    sender: Sender<ClockCommand>
}
impl ClockController {
    fn new(sender: Sender<ClockCommand>) -> Self {
        Self {
            sender
        }
    }
    pub fn start(&self){
        let _ = self.sender.send(ClockCommand::Start);
    }
    pub fn stop(&self){
        let _ = self.sender.send(ClockCommand::Stop);
    }
    pub fn set_bpm(&self, bpm: f64){
        let _ = self.sender.send(ClockCommand::ChangeBPM(bpm));
    }
    pub fn add_sender(&self, sender: Sender<ClockMessage>){
        let _ = self.sender.send(ClockCommand::AddSender(sender));
    }
}

pub struct Clock {
    bpm: f64,
    receiver: Receiver<ClockCommand>,
    senders: Vec<Sender<ClockMessage>>
}
impl Clock {
    pub fn new(bpm: f64, receiver: Receiver<ClockCommand>
    ) -> Self {
        Self {
            bpm,
            receiver,
            senders: Vec::with_capacity(8)
        }
    }
    pub fn add_sender(&mut self, sender: Sender<ClockMessage>){
        self.senders.push(sender);
    }
    pub fn set_bpm(&mut self, bpm: f64){
        self.bpm = bpm;
    }
    pub fn broadcast(&self, msg: ClockMessage){
        for sender in self.senders.iter() {
            sender.send(msg.clone());
        }
    }

}

pub fn run_clock(mut clock: Clock){
    let mut ticker = tick(Duration::from_secs_f64(60.0 / (clock.bpm * 24.0)));

    std::thread::spawn(move || {
        loop {
            select! {
                recv(ticker) -> _ => clock.broadcast(ClockMessage::Tick(Instant::now())),
                recv(clock.receiver) -> msg => {
                    match msg {
                        Ok(msg) => {
                            match msg {
                                ClockCommand::Start => (),
                                ClockCommand::ChangeBPM(bpm) => {
                                    clock.set_bpm(bpm);
                                    ticker = tick(Duration::from_secs_f64(60.0 / (bpm * 24.0)))
                                },
                                ClockCommand::AddSender(receiver) => clock.add_sender(receiver),
                                ClockCommand::Stop => (),
                            }
                        },
                        Err(_) => println!("Something went wrong with the clock")
                    }
                }
            }
        }
    });
}

pub fn build_clock() -> (ClockController, Clock) {
    let (command_sender, command_receiver) = bounded(10);

    let controller = ClockController::new(command_sender);

    let clock = Clock::new(120.0, command_receiver);

    (controller, clock)
}
