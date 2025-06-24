//! Publish/subscribe with NATS JetStream.

use anyhow::Result;

pub async fn demo() -> Result<()> {
    let client = async_nats::connect("demo.nats.io").await?;
    let js = async_nats::jetstream::new(client);

    js.publish("events.user", "hello".into()).await?;
    let mut sub = js.subscribe("events.*").await?.messages();

    if let Some(msg) = sub.next().await {
        msg.ack().await?;
        println!("received {:?}", msg.payload);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[tokio::test] async fn compiles() { assert!(true); }
}
