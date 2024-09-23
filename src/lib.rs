use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio::task;

pub trait Message: Send + 'static {}

impl<T: Send + 'static> Message for T {}

#[async_trait]
pub trait Actor: Send + Sized + 'static{
    type Msg: Message;
    async fn handle(&mut self, msg: Self::Msg, ctx: &mut Context<Self>);
}

pub struct Context<A: Actor> {
    pub addr: Addr<A>,
}

impl<A: Actor> Context<A> {
    pub fn new(addr: Addr<A>) -> Self{
        Self { addr }
    }

    pub async fn send(&self, msg: A::Msg){
        self.addr.send(msg).await;
    }

    pub async fn send_to<B: Actor>(&self, addr: Addr<B>, msg: B::Msg){
        addr.send(msg).await;
    }
}

//#[derive(Clone)]
pub struct Addr<A: Actor> {
    sender: mpsc::Sender<A::Msg>,
}

impl<A: Actor> Addr<A> {
    pub async fn send(&self, msg: A::Msg) {
        self.sender.send(msg).await;
    }
}

impl<A: Actor> Clone for Addr<A>{
    fn clone(&self) -> Self {
        Addr{ sender: self.sender.clone()}
    }
}

pub struct ActorSystem;

impl ActorSystem {
    pub fn spawn_actor<A: Actor>(mut actor: A) -> Addr<A> {
        let (tx, mut rx) = mpsc::channel::<A::Msg>(32);
        let addr = Addr { sender: tx.clone() };

        let mut ctx = Context::new(addr.clone());

        task::spawn(async move {
            for msg in rx.recv().await {
                actor.handle(msg, &mut ctx).await
            }
        });

        addr
    }
}