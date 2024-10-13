use async_trait::async_trait;
use tokio::sync::{mpsc};
pub mod context;
pub mod actor_system;

pub use actor_system::*;
pub use context::*;

/// Trait for message types that can be sent between actors.
///
/// All message types must implement the `Send` trait and have a `'static` lifetime.
pub trait Message: Send + 'static {}

impl<T: Send + 'static> Message for T {}

/// Trait representing an actor in the system.
///
/// An actor is an entity that can receive messages and process them asynchronously.
/// It must implement the `handle` method to define how to handle incoming messages.
#[async_trait]
pub trait Actor: Send + Sized + 'static {
    /// The type of messages the actor can handle.
    type Msg: Message;

    /// Handle an incoming message.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to be handled.
    /// * `ctx` - The context in which the actor operates, providing access to its address and state.
    async fn handle(&mut self, msg: Self::Msg, ctx: &mut Context<Self>);

    /// Pre-start hook for the actor.
    ///
    /// This method is called before the actor starts processing messages.
    async fn pre_start(&mut self) {

    }

    /// Post-stop hook for the actor.
    ///
    /// This method is called after the actor has stopped processing messages.
    async fn post_stop(&mut self) {

    }
}

/// Represents the address of an actor, providing a way to send messages to it.
#[derive(Debug)]
pub struct Addr<A: Actor> {
    sender: mpsc::Sender<A::Msg>,
}

impl<A: Actor> Addr<A> {
    /// Send a message to the actor associated with this address.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to send to the actor.
    pub async fn send(&self, msg: A::Msg) {
        self.sender.send(msg).await.expect("Should not fail");
    }

    /// Create a new `Addr` instance.
    ///
    /// # Arguments
    ///
    /// * `sender` - The `mpsc::Sender` used to send messages to the actor.
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

/// Trait for actors that require logging functionality.
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