## Mocha

The core api for the blog that I'm building for Jenny and me.

### Building and running

Install rust:

- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Install [docker](https://docs.docker.com/get-docker/)

Clone this repository:

- `git clone git@github.com:anish-sinha1/mocha.git`

Install cargo-watch (optional, but required if you want hot-reloading):

- `cargo install cargo-watch`

Start the containers:

- `docker compose up`

Start the app:

- `cargo run`

Or, if you want hot reloading,

- `cargo watch -x run`

### License

BSD Zero Clause License
