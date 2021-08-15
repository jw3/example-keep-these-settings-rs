use std::time::Duration;

use tokio::select;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;
use tokio::time::timeout;

pub struct System;

impl System {
    fn deploy(&self) {
        println!("deployed, pending cancellation...");
    }

    pub async fn deploy_or_kill(&self, after: Duration, kill: Receiver<()>) -> Result<(), String> {
        self.deploy();

        if let Err(_) = timeout(after, kill).await {
            println!("did not receive confirmation within {} secs, killed deployment", after.as_secs());
            Ok(())
        }
        else {
            println!("confirmed, deployment remains in place!");
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() {
    let sys = System;

    let (accept,mut on_accept) = oneshot::channel::<()>();
    let (kill, mut on_kill) = oneshot::channel::<()>();

    let deployment = sys.deploy_or_kill(Duration::from_secs(2), on_kill);

    select!{
        _ = deployment => println!("rolled back"),
        _ = on_accept => println!("accepted"),
    }
}
