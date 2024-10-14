# oricalchum

## A lightweight, high-performance actor library

## Usage

```
use async_trait::async_trait;
use oricalchum::{Actor, ActorSystem, Context};

pub enum Test {
    PrintOk(String),
    PrintErr(String, i32),
}

pub struct TestActor {
    pub name: String,
    pub value: String
}

#[async_trait]
impl Actor for TestActor {
    type Msg = Test;

    async fn handle(&mut self, msg: Self::Msg, ctx: &mut Context<Self>) {
        match msg {
            Test::PrintOk(text) => {
                println!("{} {}", self.name, text);
            }
            Test::PrintErr(text, b) => {
                println!("{} {}", text, b);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let actor1 = TestActor { name: String::from("actor1"), value: String::from("test1") };

    let addr1 = ActorSystem::spawn_actor(actor1, 16).await;

    addr1.send(Test::PrintOk(String::from("Ok"))).await;
}