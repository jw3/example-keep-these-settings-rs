use std::time::Duration;

use tokio::sync::oneshot::Receiver;
use tokio::time::timeout;

pub struct System;

impl System {
    pub fn deploy(&self) {
        println!("deployed!");
    }

    pub async fn deploy_or_kill(
        &self,
        after: Duration,
        on_accept: Receiver<()>,
    ) -> Result<(), String> {
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
