# Rust Twitch Client [![Build Status](https://travis-ci.org/mmitteregger/rust-twitch-client.svg?branch=master)](https://travis-ci.org/mmitteregger/rust-twitch-client)

## Overview

Rust Twitch Client is a library for the [Twitch REST API](https://github.com/justintv/Twitch-API) written in Rust!

It uses [hyper](https://github.com/hyperium/hyper) as http client 
and [serde](https://github.com/serde-rs/serde) for the serialization and deserialization of the REST entities.

## Installation

Until the first version of this library is released and available on [crates.io](https://crates.io) 
the dependency has to be added using the git url.

```INI
[dependencies]
twitch-client = { git = "https://github.com/mmitteregger/rust-twitch-client.git" }
```

The library currently requires Rust nightly.
Support for the stable release channel is planned for the first release.

## Examples

Getting started:

```rust
extern crate twitch_client;

use twitch_client::*;

fn main() {
    let twitch_client = TwitchClient::new();
    let top_games = twitch_client.top_games(&TopGamesParams::default()).unwrap();

    println!("Total games: {}", top_games.total());
    println!("---");
    for game_info in top_games.top() {
        println!("Game: {}, Viewers: {}", game_info.game().name(), game_info.viewers());
    }
    println!("---");
}
```

Use builders to specify arguments:

```rust
extern crate twitch_client;

use twitch_client::*;

fn main() {
    let twitch_client = TwitchClientBuilder::new()
            .client_id("<YOUR_CLIENT_ID>")
            .build();
    let params = TopGamesParamsBuilder::default()
            .offset(0)
            .limit(2)
            .build();
    let top_games = twitch_client.top_games(&params).unwrap();
    assert_eq!(top_games.top().len(), 2);

    println!("Total games: {}", top_games.total());
    println!("---");
    for game_info in top_games.top() {
        println!("Game: {}, Viewers: {}", game_info.game().name(), game_info.viewers());
    }
    println!("---");
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
