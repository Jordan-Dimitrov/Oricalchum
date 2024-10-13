use oricalchum::TrackActor;
use std::time::Duration;
use async_trait::async_trait;
use tokio::time::sleep;
use oricalchum::*;
use oricalchum_derive::TrackActor;

#[tokio::main]
async fn main() {
    let actor1 = TestActor { name: String::from("actor1"), value: String::from("test1") };
    let addr1 = ActorSystem::spawn_actor(actor1, 16).await;

    let actor2 = TestParentActor { name: String::from("actor2"), child:addr1.clone() };
    let addr2 = ActorSystem::spawn_actor(actor2, 16).await;

    addr2.send(Test::PrintOk(String::from("Ok"))).await;
    addr2.send(Test::PrintErr(String::from("Error"), 2)).await;

    sleep(Duration::from_secs(1)).await;
}


#[derive(Debug)]
pub enum Test {
    PrintOk(String),
    PrintErr(String, i32),
}

#[derive(TrackActor,Debug)]
pub struct TestActor {
    pub name: String,
    pub value: String,
}

#[derive(TrackActor,Debug)]
pub struct TestParentActor {
    pub name: String,
    pub child: Addr<TestActor>
}

#[async_trait]
impl Actor for TestParentActor {
    type Msg = Test;

    async fn handle(&mut self, msg: Self::Msg, ctx: &mut Context<Self>) {
        self.log();

        match msg {
            Test::PrintOk(text) => {
                ctx.send_to(self.child.clone(), Test::PrintOk(text)).await
            }
            Test::PrintErr(text, b) => {
                ctx.send_to(self.child.clone(), Test::PrintErr(text, b)).await
            }
        }
    }

    async fn post_stop(&mut self) {
        println!("Stopped");
    }
}

#[async_trait]
impl Actor for TestActor {
    type Msg = Test;

    async fn handle(&mut self, msg: Self::Msg, ctx: &mut Context<Self>) {
        self.log();

        match msg {
            Test::PrintOk(text) => {
                println!("{} {}", self.name, text);
            }
            Test::PrintErr(text, b) => {
                println!("{} {}", text, b);
                ctx.terminate();
            }
        }
    }

    async fn post_stop(&mut self) {
        println!("Stopped");
    }
}