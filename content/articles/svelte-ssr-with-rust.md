---
title: Svelte Server-side Rendering with Rust
slug: svelte-ssr-with-rust
status: unpublished
published: 2022-11-26 00:00
excerpt: A how to guide on rendering Svelte JavaScript on the server-side using Rust and the Axum web framework
tags:
  - axum
  - svelte
  - ssr
---
# How to Use Rust and Axum to Render Svelte on the Server-Side

In this how-to guide, we're going to explore using Rust and the Axum web framework to render Svelte on the server-side. First we'll set up a Rust REST API backend using Axum, and a Svelte frontend application to render data from the API. After, we'll Finally, we'll set up server-side rendering so our pages can be crawled by bots as well as load very fast.

## Setting up our Rust REST API
First we'll set up a Rust REST API using the Axum web framework. We'll create a project called My Dogs, and install some dependencies.


```bash
cargo init my-dogs
cd my-dogs
cargo add tokio --features full
cargo add serde --features serde_derive
cargo add serde_json axum
```

Next let's create a basic REST API that will return some dog information for our blog to render.

```rust
use axum::{
  routing::get,
  http::StatusCode,
  response::IntoResponse,
  Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

mod api {
  use super::*;

  #[derive(Serialize, Deserialize)]
  pub enum Breed {
    BostonTerrier,
    Boxer,
    Vizsla,
  }

  #[derive(Serialize, Deserialize)]
  pub struct Dog {
    pub name: String,
    pub photo: String,
    pub biography: String,
    pub breed: Vec<Breed>,
  }

  pub async fn dogs() -> impl IntoResponse {
    let dogs: Vec<Dog> = vec![
      Dog {
        name: "Puck".into(),
        breed: vec![Breed::BostonTerrier, Breed::Boxer],
      },
      Dog {
        name: "Oliver".into(),
        breed: vec![Breed::Vizsla],
      },
      Dog {
        name: "Walter".into(),
        breed: vec![Breed::Vizsla],
      }
    ];

    Json(dogs)
  }
}

#[tokio::main]
async fn main() {
  let app = Router::new()
    .route("/api/dogs", get(api::dogs));

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  println!("listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
```

Now we can run our Rust REST API backend from the command line.

```bash
cargo run
listening on 127.0.0.1:3000
```

And we can check that our API is returning results:

```json
http localhost:3000/api/dogs
HTTP/1.1 200 OK
content-length: 125
content-type: application/json
date: Sun, 27 Nov 2022 01:04:12 GMT

[
  {
    "breed": [
      "BostonTerrier",
      "Boxer"
    ],
    "name": "Puck"
  },
  {
    "breed": [
      "Vizsla"
    ],
    "name": "Oliver"
  },
  {
    "breed": [
      "Vizsla"
    ],
    "name": "Walter"
  }
]
```

Great! Now we need a frontend to display cute pictures of my dogs.

#blog/unpublished/rust