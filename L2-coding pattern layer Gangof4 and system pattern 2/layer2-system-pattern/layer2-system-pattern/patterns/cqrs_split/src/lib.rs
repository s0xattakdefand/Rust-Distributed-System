//! In-proc CQRS demo: command bus + read model channel.

use tokio::sync::{broadcast, mpsc};

#[derive(Debug, Clone)]
pub enum Command { Create(i32), Delete(i32) }
#[derive(Debug, Clone)]
pub enum Event   { Created(i32), Deleted(i32) }

pub struct CommandBus { tx: mpsc::Sender<Command> }
impl CommandBus {
    pub fn new(event_tx: broadcast::Sender<Event>) -> Self {
        let (tx, mut rx) = mpsc::channel(32);
        tokio::spawn(async move {
            while let Some(cmd) = rx.recv().await {
                match cmd {
                    Command::Create(id) => { let _ = event_tx.send(Event::Created(id)); }
                    Command::Delete(id) => { let _ = event_tx.send(Event::Deleted(id)); }
                }
            }
        });
        Self{tx}
    }
    pub async fn dispatch(&self,c:Command){let _=self.tx.send(c).await;}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test] async fn flow() {
        let (etx, mut erx) = broadcast::channel(16);
        let bus = CommandBus::new(etx);
        bus.dispatch(Command::Create(1)).await;
        assert!(matches!(erx.recv().await.unwrap(), Event::Created(1)));
    }
}
