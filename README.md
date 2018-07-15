# ssb-pub

ssb-pub is a worked example of a web service and Secure Scuttlebutt pub server written in Rust.

---

## Features

  * HTTP server for management and non-ssb API using [actix-web](https://github.com/actix/actix-web)
  * Hierarchical configuration system thanks to [config](https://github.com/mehcode/config-rs)
  * Logging management using [slog](https://github.com/slog-rs/slog)
  * [tokio](https://github.com/tokio-rs/tokio) and [futures](https://crates.io/crates/futures) for asynchronous streaming
  * This doesn't yet implement any Secure Scuttlebutt features

## Usage

You can build and run the server by calling:

```shell
$ cargo run --example server
    Finished dev [unoptimized + debuginfo] target(s) in 1.25s
     Running `target/debug/examples/server`
version: 0.1.0
 Jul 16 13:19:43.877 INFO Listening on 127.0.0.1:3123
 Jul 16 13:19:43.882 INFO Application started
 Jul 16 13:19:43.883 INFO MOTD is Hello world!
 Jul 16 13:19:43.884 DEBG loop process - 1 events, 0.000s
 Jul 16 13:19:43.885 DEBG loop process - 1 events, 0.000s
```

### Configuration

Configuration defaults are available in `examples/default.toml`. Values can be overridden using a `config.toml`.

```shell
$ cat > config.toml <<EOF
[web]
motd = "My awesome message of the day!!!"
EOF
$ cargo run --example server
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/examples/server`
version: 0.1.0
 Jul 16 13:23:35.969 INFO Listening on 127.0.0.1:3123
 Jul 16 13:23:35.974 INFO Application started
 Jul 16 13:23:35.974 INFO MOTD is My awesome message of the day!!!
 Jul 16 13:23:35.975 DEBG loop process - 1 events, 0.000s
 Jul 16 13:23:35.975 DEBG loop process - 1 events, 0.000s
```

Configuration can also be overridden using environment variables.

```shell
$ APP_SERVER__LOG_LEVEL=info APP_WEB__MOTD="Foo?" cargo run --example server
    Finished dev [unoptimized + debuginfo] target(s) in 1.38s
     Running `target/debug/examples/server`
version: 0.1.0
 Jul 16 13:31:35.534 INFO Listening on 127.0.0.1:3123
 Jul 16 13:31:35.535 INFO Application started
 Jul 16 13:31:35.535 INFO MOTD is Foo?
```

## Contributing

All contributions are welcome, if you notice any bugs please open an issue!

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
