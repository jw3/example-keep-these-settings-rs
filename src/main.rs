use std::io::Write;
use std::time::Duration;
use std::{io, thread};

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tokio::select;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;
use tokio::time::timeout;

pub struct System;

impl System {
    fn deploy(&self) {
        println!("deployed!");
    }

    pub async fn deploy_or_kill(&self, after: Duration, kill: Receiver<()>) -> Result<(), String> {
        self.deploy();
        println!("press Enter within {} seconds to keep...", after.as_secs());

        if let Err(_) = timeout(after, kill).await {
            println!("did not receive confirmation, cancelling deployment");
            Ok(())
        } else {
            println!("confirmed, deployment remains in place!");
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() {
    let sys = System;

    let (accept, mut on_accept) = oneshot::channel::<()>();
    let (kill, mut on_kill) = oneshot::channel::<()>();

    let mut stdin = termion::async_stdin().keys();
    thread::spawn(move || loop {
        if let Some(Ok(key)) = stdin.next() {
            match key {
                _ => {
                    accept.send(());
                    return;
                }
            }
        }
        thread::sleep(Duration::from_millis(50));
    });

    let deployment = sys.deploy_or_kill(Duration::from_secs(6), on_kill);
    select! {
        _ = deployment => println!("rolling back deployment"),
        _ = on_accept => println!("accepted, deployment remains"),
    }
}
