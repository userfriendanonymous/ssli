use std::sync::Arc;

use clap::Parser;
use s2rs::api::Tokens;
use store::{Store};
use session::Session;

mod input;
mod store;
mod session;
mod output;


#[tokio::main]
async fn main() {
    let input = input::Input::parse();

    let store = Arc::new(Store::new().unwrap());
    let main_session = store.main_session().await;

    let scratch = s2rs::Api::with_auth(main_session.name, &Tokens {
        csrf: "a".to_owned(),
        session: main_session.session,
        x: main_session.x
    }).unwrap();

    let session = Arc::new(Session::new(store, scratch));

    let output = input.run(session).await;
    println!("{}", output.finish(0));
}