//! WebSocket broadcast hub stub.

use tokio::sync::broadcast;

pub struct Hub<T: Clone> {
    tx: broadcast::Sender<T>,
}
impl<T: Clone> Hub<T> {
    pub fn new(cap:usize)->Self{
        let (tx,_)=broadcast::channel(cap); Self{tx}
    }
    pub fn publish(&self,msg:T){let _=self.tx.send(msg);}
    pub fn subscribe(&self)->broadcast::Receiver<T>{self.tx.subscribe()}
}

#[cfg(test)]
mod tests{
    use super::*;#[tokio::test]async fn fan_out(){
        let h=Hub::new(8);
        let mut a=h.subscribe(); let mut b=h.subscribe();
        h.publish("x");
        assert_eq!(a.recv().await.unwrap(),"x");
        assert_eq!(b.recv().await.unwrap(),"x");
    }
}
