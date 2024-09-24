use std::time::Duration;
use async_trait::async_trait;
use tokio::sync::{mpsc, Mutex};
use tokio::task;
use tokio::time::sleep;

pub trait Message: Send + 'static {}

impl<T: Send + 'static> Message for T {}

#[async_trait]
pub trait Actor: Send + Sized + 'static{
    type Msg: Message;
    async fn handle(&mut self, msg: Self::Msg, ctx: &mut Context<Self>);
}

#[derive(PartialEq, Debug, Clone)]
pub enum ActorState {
    Running,
    Terminated
}

pub struct Context<A: Actor> {
    pub addr: Addr<A>,
    mailbox_size: usize,
    state: Mutex<ActorState>
}

impl<A: Actor> Context<A> {
    pub fn new(addr: Addr<A>, mailbox_size: usize) -> Self {
        Self { addr, mailbox_size, state: Mutex::new(ActorState::Running) }
    }

    pub async fn send(&self, msg: A::Msg) {
        let state = self.state.lock().await;

        if *state == ActorState::Running {
            self.addr.send(msg).await;
        } else {
            sleep(Duration::from_millis(1)).await;
        }
    }

    pub async fn send_to<B: Actor>(&self, addr: Addr<B>, msg: B::Msg) {
        let state = self.state.lock().await;

        if *state == ActorState::Running {
            addr.send(msg).await;
        } else {
            sleep(Duration::from_millis(1)).await;
        }
    }

    pub fn get_mailbox_size(&self) -> usize{
        self.mailbox_size
    }

    pub async fn terminate(&self) {
        let mut state = self.state.lock().await;
        *state = ActorState::Terminated;
    }

    pub async fn get_state(&self) -> ActorState{
        let state = self.state.lock().await;
        state.clone()
    }
}

//#[derive(Clone)]
pub struct Addr<A: Actor> {
    sender: mpsc::Sender<A::Msg>,
}

impl<A: Actor> Addr<A> {
    pub async fn send(&self, msg: A::Msg) {
        self.sender.send(msg).await.expect("should not fail");
    }
}

impl<A: Actor> Clone for Addr<A> {
    fn clone(&self) -> Self {
        Addr{ sender: self.sender.clone()}
    }
}

pub struct ActorSystem;
impl ActorSystem {
    pub fn spawn_actor<A: Actor>(mut actor: A, mailbox_size: usize) -> Addr<A> {
        let (tx, mut rx) = mpsc::channel::<A::Msg>(mailbox_size);
        let addr = Addr { sender: tx.clone() };

        let mut ctx = Context::new(addr.clone(), mailbox_size);

        task::spawn(async move {
            while let Some(msg) = rx.recv().await {
                let state = ctx.get_state().await;

                match state {
                    ActorState::Running => actor.handle(msg, &mut ctx).await,
                    ActorState::Terminated => {
                        drop(actor);
                        break;
                    }
                }
            }
        });

        addr
    }
}
