
use warp::Filter;
use tokio;
use iridium_client;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let verifier = iridium_client::generate_random_string();
    let auth_verifier = verifier.clone();
    let callback_verifier = verifier.clone();


    let auth = warp::path("auth").map(move || {
        let uri = iridium_client::authenticate_with_external_redirect(auth_verifier.clone());
            warp::redirect(uri.unwrap())
    });


    let callback = warp::path("callback").and(warp::query::<HashMap<String, String>>()).and_then( move |params: HashMap<String, String>| {
        let response =  iridium_client::callback_service::callback_service::handle_callback(params, callback_verifier.clone());
        async {
            if let Ok(res) = response.await {
                if let Ok(user) = iridium_client::get_identity(&res.token).await {
                    println!("user id: {}, username: {}", user.data.id, user.data.username);
                    Ok(user)
                } else {
                    eprintln!("Error getting identity");
                    Err(warp::reject())
                }

            } else {
                eprintln!("Error handling callback");
                Err(warp::reject())
            }

        }
    });

    let routes = auth.or(callback);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
