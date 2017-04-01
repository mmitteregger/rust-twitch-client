# TODO

 * Change the API to require a [Client-ID](https://blog.twitch.tv/client-id-required-for-kraken-api-calls-afbb8e95f843) to construct the TwitchClient
 * Add Client-ID to Travis to fix tests and build with stable and beta versions without ignoring failures
 * Migrate try!() macros to the [? operator](https://blog.rust-lang.org/2016/11/10/Rust-1.13.html#the--operator)
 * Use [error_chain](https://docs.rs/error-chain) (and maybe change error values)
 * Consider using [reqwest](https://docs.rs/reqwest)
 * Update to the latest [Twitch API](https://dev.twitch.tv/docs)
 * Extend the client
