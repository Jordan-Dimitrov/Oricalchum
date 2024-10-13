use tokio::sync::mpsc;
use tokio::task;
use crate::{Actor, ActorState, Addr, Context};

pub struct ActorSystem;
impl ActorSystem {
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