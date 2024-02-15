pub mod assertion_parser;
pub mod ipc_assertion;
pub mod ipc;

use std::{
    process::Command,
    sync::mpsc::{channel, Receiver},
    thread::sleep,
    time::Duration,
};

use interactive_process::InteractiveProcess;
use ipc::{ArwenMessage, ElrondMessage};

pub struct Elrond {
    proc: InteractiveProcess,
    receiver: Receiver<ElrondMessage>,
}

impl Elrond {
    pub fn new() -> Elrond {
        let mut binding = Command::new("dune");
        let cmd = binding
            .arg("exec")
            .arg("stub")
            .arg("--profile")
            .arg("release");

        let (sender, receiver) = channel();

        let proc = InteractiveProcess::new(cmd, move |line| {
            let line = line.unwrap();
            println!("Got: {}", line);

            sender.send(serde_json::from_str(&line).unwrap()).unwrap();
        })
        .unwrap();

        Elrond { proc, receiver }
    }

    pub fn send_message(&mut self, msg: ArwenMessage) -> Result<(), String> {
        let serialized = serde_json::to_string(&msg).unwrap();
        eprintln!("Sending: {}", serialized);

        self.proc.send(&serialized).unwrap();
        Ok(())
    }

    pub fn receive_message(&mut self) -> ElrondMessage {
        self.receiver.recv().unwrap()
    }

    pub fn kill(self) -> Result<(), String> {
        sleep(Duration::from_secs(1));
        Ok(())
    }
}

impl Default for Elrond {
    fn default() -> Self {
        Self::new()
    }
}
