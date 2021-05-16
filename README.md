
# Confu

No frills app configuration via *environment* or *command line arguments* for
software written in Rust.

Why Confu? Geared towards microservices, and has minimal direct dependencies
list: `syn`, `quote`, `proc-macro2`.

if a more user friendly command line parsing desired, there are great and
proven crate alternatives. For example, [Clap](https://lib.rs/crates/clap).

## Features

- at compile time (when a binary is produced for your app), captures
  - build type, i.e. `debug` or `release`
  - build version, if provided via environment variable `[PREFIX]VERSION`,
    otherwise is set to `<unspecified>`
- Reads configuration from either
  - environment
  - command line arguments
  - defaults
- configuration items may have an optional prefix like `APP_`
- each config item can be
  - *required* - if not provided, will `panic`
  - *protected* - will display "`xxxxxxx`" instead of sensitive information
  - *hidden* - will not be displayed at all
- Specificity: defaults -> environment -> arguments. Arguments being the most specific,
  will take precedence over the corresponding environment values, if such are also defined

## Usage/Examples

A working [example](https://github.com/izirku/confu/tree/main/examples/basic) is provided
in repository. And a quick usage summary here as well:

In `Cargo.toml`:

```toml
[dependencies]
confu = "*"
```

then, a code like this:

```rust
use confu::Confu;
use std::env

#[derive(Confu)]
#[confu_prefix = "APP_"]
struct Config {
    #[default = "postgres"]
    db_user: String,

    #[protect]
    #[default = "postgres"]
    db_password: String,

    #[default = "127.0.0.1"]
    api_host: String,

    #[require]
    telemetry: String,

    #[hide]
    super_secret_stuff: String,
}

fn main() {
    let config = Config::confu();
    config.show();
}
```

should produce something like this, granted that `APP_VERSION="0.1.0"`
environment variable is also set:

```bash
$ cargo run --quiet -- --app_telemetry=yes
  build: debug
version: 0.1.0

APP_DB_USER/--app_db_user=postgres  (default: "postgres")
APP_DB_PASSWORD/--app_db_password=xxxxxxx  (default: "xxxxxxx")
APP_API_HOST/--app_api_host=127.0.0.1  (default: "127.0.0.1")
APP_TELEMETRY/--app_telemetry=yes  (required)
```

if a required argument was omitted, a `panic` will occur:

```bash
$ cargo run --quiet
thread 'main' panicked at 'required argument APP_TELEMETRY/--app_telemetry was not provided.', examples\basic\src\config.rs:4:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Roadmap

- [x] Write the documentation
- [ ] Write the tests
- [ ] Produce a better error reporting in macros
- [ ] Parse into numerical and `bool` types

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Confu by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
