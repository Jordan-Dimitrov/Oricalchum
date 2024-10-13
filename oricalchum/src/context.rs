use std::time::Duration;
use tokio::time::sleep;
use crate::{Actor, Addr};

pub struct Context<A: Actor> {
    addr: Addr<A>,
    mailbox_size: usize,
    state: ActorState
}

#[derive(PartialEq, Debug, Clone)]
pub enum ActorState {
    Running,
    Terminated
}

impl<A: Actor> Context<A> {
    pub fn new(addr: Addr<A>, mailbox_size: usize) -> Self {
        Self { addr, mailbox_size, state: ActorState::Running }
    }

    pub async fn send(&self, msg: A::Msg) {

        if self.state == ActorState::Running {
            self.addr.send(msg).await;
        } else {
            sleep(Duration::from_millis(1)).await;
        }
    }

    pub async fn send_to<B: Actor>(&self, addr: Addr<B>, msg: B::Msg) {

        if self.state == ActorState::Running {
            addr.send(msg).await;
        } else {
            sleep(Duration::from_millis(1)).await;
        }
    }

    pub fn get_mailbox_size(&self) -> usize{
        self.mailbox_size
    }

    pub fn terminate(&mut self) {
        self.state = ActorState::Terminated;
    }

    pub fn get_state(&self) -> ActorState{
        self.state.clone()
    }
}
