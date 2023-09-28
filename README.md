# Getting Started:

*initialize cargo project*:

`$ cargo new <your_project_name> --bin`

*add `iridium-client`*:

`$ cargo add iridium-client`

*add dependencies*:

`$ cargo add warp tokio`

*src/main.rs*:

Currently, the client can authenticate/authorize a `warp` project. You need the 3 functions to do so are `authenticate_with_external_redirect`, `handle_callback`, `get_identity`.

First, you need to use `authenticate_with_external_redirect` within a "auth" path. This will redirect the client to Iridium's core service.

```rust
fn main() {

  let auth = warp::path("auth").map(move || {
        let uri = iridium_client::authenticate_with_external_redirect(auth_verifier.clone());
            warp::redirect(uri.unwrap())
    });
}
```

Then, you need to implement a "callback" path and call the `handle_callback` function.

```rust
 let callback = warp::path("callback").and(warp::query::<HashMap<String, String>>()).and_then( move |params: HashMap<String, String>| {
        let response =  iridium_client::callback_service::callback_service::handle_callback(params, callback_verifier.clone());
}
```

`handle_callback` returns an `Future<Option<String>>` the response will contain the access token.

`await` and use the access token with `get_identity()`

```rust
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
```

Happy Hacking!
