use clap::Parser;
use s2rs::api::Tokens;
use store::{Store};

mod input;
mod store;
mod session;
mod output;


#[tokio::main]
async fn main() {
    let input = input::Input::parse();

    let store = Store::new().unwrap();
    let main_session = store.main_session().await;

    let session = s2rs::Session::with_auth(main_session.name, &Tokens {
        csrf: "a".to_owned(),
        session: main_session.session,
        x: main_session.x
    }).unwrap();

    let output = session::entry(input, store, session).await.unwrap_or_else(|e| e);
    println!("{}", output.finish());
}