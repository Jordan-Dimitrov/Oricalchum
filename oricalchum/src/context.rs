use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;
use crate::{Actor, ActorState, Addr};

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
