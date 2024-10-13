use tokio::sync::mpsc;
use tokio::task;
use crate::{Actor, ActorState, Addr, Context};

/// Represents the actor system responsible for managing actors.
///
/// This struct provides methods to spawn new actors and manage their lifecycle.
pub struct ActorSystem;
impl ActorSystem {
    /// Spawns a new actor within the actor system.
    ///
    /// This method initializes a new actor with the specified mailbox size,
    /// sets up a communication channel, and starts the actor's task.
    ///
    /// # Type Parameters
    ///
    /// * `A` - The type of the actor being spawned, which must implement the `Actor` trait.
    ///
    /// # Arguments
    ///
    /// * `actor` - An instance of the actor to be spawned.
    /// * `mailbox_size` - The size of the mailbox for storing messages intended for the actor.
    ///
    /// # Returns
    ///
    /// An `Addr<A>` representing the address of the newly spawned actor.
    ///
    /// # Examples
    ///
    /// ```
    /// use oricalchum::ActorSystem;
    /// let my_actor = MyActor {};
    /// let addr = ActorSystem::spawn_actor(my_actor, 10).await;
    pub async fn spawn_actor<A: Actor>(mut actor: A, mailbox_size: usize) -> Addr<A> {
        let (tx, mut rx) = mpsc::channel::<A::Msg>(mailbox_size);
        let addr = Addr::new(tx.clone());

        let mut ctx = Context::new(addr.clone(), mailbox_size);

        task::spawn(async move {
            actor.pre_start().await;
            while let Some(msg) = rx.recv().await {
                let state = ctx.get_state();

                match state {
                    ActorState::Running => actor.handle(msg, &mut ctx).await,
                    ActorState::Terminated => {
                        actor.post_stop().await;
                        drop(actor);
                        break;
                    }
                }
            }
        });

        addr
    }
}