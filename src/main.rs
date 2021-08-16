use std::thread;
use std::time::Duration;

use termion;
use termion::input::TermRead;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;
use tokio::time::timeout;

struct System;

impl System {
    fn deploy(&self) {
        println!("deployed!");
    }

    async fn deploy_or_kill(&self, after: Duration, on_accept: Receiver<()>) -> Result<(), String> {
        self.deploy();
        println!(
            "press Enter within {} seconds or will rollback...",
            after.as_secs()
        );

        if let Err(_) = timeout(after, on_accept).await {
            println!("not accepted, rolling back deployment");
        } else {
            println!("accepted, deployment remains")
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let sys = System;
    let (accept, on_accept) = oneshot::channel::<()>();

    let mut stdin = termion::async_stdin().keys();
    thread::spawn(move || loop {
        if let Some(Ok(key)) = stdin.next() {
            match key {
                _ => {
                    accept.send(()).expect("failed accept");
                    return;
                }
            }
        }
        thread::sleep(Duration::from_millis(50));
    });

    sys.deploy_or_kill(Duration::from_secs(6), on_accept)
        .await
        .unwrap();
}
