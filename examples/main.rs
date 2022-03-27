use std::thread;
use std::time::Duration;

use termion;
use termion::input::TermRead;
use tokio::sync::oneshot;

use libkeep_these::sys::System;

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
