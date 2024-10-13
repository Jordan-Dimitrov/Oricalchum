use async_trait::async_trait;
use tokio::sync::{mpsc};
pub mod context;
pub mod actor_system;

pub use actor_system::*;
pub use context::*;

pub trait Message: Send + 'static {}

impl<T: Send + 'static> Message for T {}

#[async_trait]
pub trait Actor: Send + Sized + 'static {
    type Msg: Message;
    async fn handle(&mut self, msg: Self::Msg, ctx: &mut Context<Self>);
    async fn pre_start(&mut self) {

    }
    async fn post_stop(&mut self) {

    }
}

//#[derive(Clone)]
pub struct Addr<A: Actor> {
    sender: mpsc::Sender<A::Msg>,
}

impl<A: Actor> Addr<A> {
    pub async fn send(&self, msg: A::Msg) {
        self.sender.send(msg).await.expect("Should not fail");
    }

    pub fn new(sender: mpsc::Sender<A::Msg>) -> Self
    {
        Addr{ sender }
    }

}

impl<A: Actor> Clone for Addr<A> {
    fn clone(&self) -> Self {
        Addr{ sender: self.sender.clone()}
    }
}

pub trait TrackActor: Actor {
    fn log(&self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestMessage {
        name: String
    }

    struct TestActor {
    }

    #[async_trait]
    impl Actor for TestActor {
        type Msg = TestMessage;

        async fn handle(&mut self, msg: Self::Msg, _ctx: &mut Context<Self>) {
            println!("{:?}", msg.name);
        }

        async fn pre_start(&mut self) {
            println!("Starting");
        }

        async fn post_stop(&mut self) {
            println!("Stooped");
        }
    }

    #[tokio::test]
    async fn test_actor_state() {
        let actor = TestActor {};

        let addr = ActorSystem::spawn_actor(actor, 10).await;

        let mut ctx = Context::new(addr.clone(), 10);

        assert_eq!(ctx.get_state(), ActorState::Running);

        ctx.terminate();

        assert_eq!(ctx.get_state(), ActorState::Terminated);
    }
}