use std::time::Duration;
use tokio::time::sleep;
use crate::{Actor, Addr};

/// Represents the context in which an actor operates.
///
/// This structure holds information about the actor's address, mailbox size, and state.
pub struct Context<A: Actor> {
    addr: Addr<A>,
    mailbox_size: usize,
    state: ActorState
}

/// Possible states of an actor.
#[derive(PartialEq, Debug, Clone)]
pub enum ActorState {
    Running,
    Terminated
}

impl<A: Actor> Context<A> {
    /// Creates a new `Context` for the specified actor.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address of the actor.
    /// * `mailbox_size` - The size of the mailbox for storing messages.
    ///
    /// # Returns
    ///
    /// A new instance of `Context` initialized with the given address and mailbox size.
    pub fn new(addr: Addr<A>, mailbox_size: usize) -> Self {
        Self { addr, mailbox_size, state: ActorState::Running }
    }

    /// Sends a message to the actor associated with this context.
    ///
    /// The message will only be sent if the actor is in the `Running` state. If the actor is `Terminated`,
    /// the method will wait for a short duration before returning.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to send to the actor.
    pub async fn send(&self, msg: A::Msg) {

        if self.state == ActorState::Running {
            self.addr.send(msg).await;
        } else {
            sleep(Duration::from_millis(1)).await;
        }
    }

    /// Sends a message to a different actor.
    ///
    /// The message will only be sent if the target actor is in the `Running` state. If the actor is `Terminated`,
    /// the method will wait for a short duration before returning.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address of the target actor to send the message to.
    /// * `msg` - The message to send to the target actor.
    pub async fn send_to<B: Actor>(&self, addr: Addr<B>, msg: B::Msg) {

        if self.state == ActorState::Running {
            addr.send(msg).await;
        } else {
            sleep(Duration::from_millis(1)).await;
        }
    }

    /// Returns the maximum size of the actor's mailbox.
    ///
    /// # Returns
    ///
    /// The size of the mailbox as a `usize`.
    pub fn get_mailbox_size(&self) -> usize{
        self.mailbox_size
    }

    /// Terminates the actor's context, changing its state to `Terminated`.
    ///
    /// Once terminated, the actor will no longer process any messages.
    pub fn terminate(&mut self) {
        self.state = ActorState::Terminated;
    }

    /// Retrieves the current state of the actor.
    ///
    /// # Returns
    ///
    /// The current state of the actor as an `ActorState`.
    pub fn get_state(&self) -> ActorState{
        self.state.clone()
    }
}
