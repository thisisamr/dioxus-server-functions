## Overview

Welcome to Dioxus-Fullstack! This repository provides server functions that enable seamless communication between the client and server, treating server calls as if they were local functions. This README will guide you through setting up and using these functionalities.

## Table of Contents

- [Setting Up a Server Function](#setting-up-a-server-function)
- [Cached Data Fetching](#cached-data-fetching)
- [Running the Client with Dioxus-Desktop](#running-the-client-with-dioxus-desktop)
- [Client Code](#client-code)
- [Server Code](#server-code)

## Setting Up a Server Function

Dioxus-Fullstack allows you to create server functions with ease. To create a server function, follow these steps:

1. Add the `#[server(YourUniqueType)]` attribute to your function.
2. Ensure your function is asynchronous (async).
3. The function's arguments and return type must implement `serialize` and `deserialize` with serde.
4. Return a `Result` with an error type of `ServerFnError`.

Example:

```rust
#[server]
async fn double_server(number: i32) -> Result<i32, ServerFnError> {
    // Perform server-side computation or access a database
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let result = number * 2;
    println!("Server calculated: {result}");
    Ok(result)
}
```
Remember to call register on the type passed into the server macro in your main function before starting your server.

Cached Data Fetching
Dioxus-Fullstack also supports cached data fetching for improved performance. Use the use_server_future hook to wait for a future on the server and send the result down to the client.

Example:

```rust
#[server]
async fn get_server_data() -> Result<String, ServerFnError> {
    // Access a database
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    Ok("Hello from the server!".to_string())
}
```
In your client code:

```rust
let mut count = use_server_future(cx, (), |_| async { get_server_data().await })?;
```
The ? after use_server_future instructs Dioxus-Fullstack to wait for the future to resolve before continuing rendering.

Running the Client with Dioxus-Desktop
Dioxus-Fullstack supports interaction with a desktop program in a similar fashion as with a web browser. Follow these steps:

Create two binary targets - one for the desktop program (client.rs) and one for the server (server.rs).
Configure build settings in Cargo.toml to include necessary dependencies and features.
Run the server executable with cargo run --bin server --features ssr.
Run the client desktop executable with cargo run --bin client --features desktop.
Client Code
The client code is straightforward. Set the server URL in the client code, and Dioxus-Desktop will launch the app.

```rust
// Set the server URL
let server_url = "http://localhost:8080";

// Launch the app
dioxus_desktop::launch(App, &server_url);
```
Remember to update the URL for production.

Server Code
In the server code, set the network address and port where the server will listen to:

```rust
let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
Register the types declared in the server function macros into the Axum server:
```

```rust
let _ = GetServerData::register_explicit();
Start the server, and it will begin responding to requests.
```
Feel free to explore the full example code in the Dioxus repository for a comprehensive understanding of Dioxus-Fullstack capabilities.
