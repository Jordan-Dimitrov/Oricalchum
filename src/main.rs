use oricalchum::TrackActor;
use std::time::Duration;
use async_trait::async_trait;
use tokio::time::sleep;
use oricalchum::*;
use oricalchum_derive::TrackActor;

#[tokio::main]
async fn main() {
    let actor1 = TestActor { name: String::from("actor1"), value: String::from("test1") };
    let actor2 = TestActor { name: String::from("actor2"), value: String::from("test2") };

    let addr1 = ActorSystem::spawn_actor(actor1, 16).await;
    let addr2 = ActorSystem::spawn_actor(actor2, 16).await;

    addr1.send(Test::PrintOk(String::from("Valjo"))).await;
    
    addr2.send(Test::PrintErr(String::from("Nije valjo"), 2)).await;

    addr2.send(Test::PrintOk(String::from("Valjo"))).await;

    sleep(Duration::from_secs(1)).await;
}


#[derive(Debug)]
pub enum Test {
    PrintOk(String),
    PrintErr(String, i32),
}

#[derive(TrackActor)]
pub struct TestActor {
    pub name: String,
    pub value: String
}

#[async_trait]
impl Actor for TestActor {
    type Msg = Test;

    async fn handle(&mut self, msg: Self::Msg, ctx: &mut Context<Self>) {
        //let addr3 = ActorSystem::spawn_actor(TestActor { name: String::from("actor3") } , 16);
        //ctx.send_to(addr3, Test::PrintOk(String::from("Valjo"))).await;
        //self.test().await;

        self.log();
        match msg {
            Test::PrintOk(text) => {
                println!("{} {}", self.name, text);
            }
            Test::PrintErr(text, b) => {
                println!("{} {}", text, b);
                ctx.terminate().await;
            }
        }

        sleep(Duration::from_nanos(1)).await;

        //exit(1);
    }

    async fn post_stop(&mut self) {
        println!("treba da odmori");
    }
}