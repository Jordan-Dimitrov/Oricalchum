use std::process::exit;
use std::time::Duration;
use async_trait::async_trait;
use tokio::time::sleep;
use oricalchum::{Actor, ActorSystem, Context};

#[tokio::main]
async fn main() {
    let addr1 = ActorSystem::spawn_actor(TestActor { name: String::from("actor1") });
    let addr2 = ActorSystem::spawn_actor(TestActor { name: String::from("actor2") });

    addr1.send(Test::PrintOk(String::from("Valjo"))).await;
    addr2.send(Test::PrintErr(String::from("Nije valjo"), 2)).await;

    sleep(Duration::from_secs(1)).await;
}


#[derive(Debug)]
pub enum Test {
    PrintOk(String),
    PrintErr(String, i32),
}

pub struct TestActor {
    pub name: String,
}

#[async_trait]
impl Actor for TestActor {
    type Msg = Test;

    async fn handle(&mut self, msg: Self::Msg, ctx: &mut Context<Self>) {
        let addr3 = ActorSystem::spawn_actor(TestActor { name: String::from("actor3") } );
        ctx.send_to(addr3, Test::PrintOk(String::from("Valjo"))).await;

        match msg {
            Test::PrintOk(text) => {
                println!("{} {}", self.name, text);
            }
            Test::PrintErr(text, b) => {
                println!("{} {}", text, b);
            }
        }

        sleep(Duration::from_millis(1)).await;
        exit(1);
    }
}