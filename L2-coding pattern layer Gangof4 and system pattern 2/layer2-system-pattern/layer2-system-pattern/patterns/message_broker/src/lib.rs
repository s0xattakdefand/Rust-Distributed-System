//! Tiny broker built on Tokio mpsc.

use tokio::sync::broadcast;

pub fn channel<T: Clone>(cap:usize) -> (broadcast::Sender<T>, broadcast::Receiver<T>) {
    broadcast::channel(cap)
}

#[cfg(test)]
mod tests {
    use super::*; #[tokio::test] async fn pub_sub(){
        let (tx, mut rx) = channel::<&str>(16);
        tx.send("hi").unwrap();
        assert_eq!(rx.recv().await.unwrap(),"hi");
    }
}
