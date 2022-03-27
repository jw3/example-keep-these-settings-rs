use core::time::Duration;
use std::future::Future;
use std::thread;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;
use tokio::time::{timeout, Timeout};

struct PyTimer {
    running: Option<Running>,
}

struct Running {
    t: Timeout<Receiver<()>>,
}

impl Default for PyTimer {
    fn default() -> Self {
        Self { running: None }
    }
}

// do you want to keep these settings?
impl PyTimer {
    fn new() -> Self {
        Self::default()
    }

    async fn start(&mut self) {
        let (accept, on_accept) = oneshot::channel::<()>();
        let t = timeout(Duration::from_secs(6), on_accept);

        self.running = Some(Running { t })
    }

    // Q: dywtkts?
    // A: No
    fn preempt(&mut self) {
        match std::mem::replace(&mut self.running, None) {
            Some(running) => {
                running.t.into_inner();
                self.stop_daemon();
            }
            None => {
                // todo;; should python::throw here
                println!("timer was not started")
            }
        }
    }

    // Q: dywtkts?
    // A: Yes
    fn cancel(&mut self) {
        match std::mem::replace(&mut self.running, None) {
            Some(running) => {
                running.t.into_inner();
            }
            None => {
                // todo;; should python::throw here
                println!("timer was not started")
            }
        }
    }

    fn stop_daemon(&self) {
        println!("stop the daemon!")
    }
}
