
use iridium_client;
use warp;
use tokio;
use warp::Filter;


#[tokio::main]
fn main() {
    let verifier = iridium_client::generate_random_string();
    let auth_verifier = &verifier.clone();
    let callback_verifier = &verifier.clone();

    let auth = warp::path!("auth").map(move || {

    });

}
