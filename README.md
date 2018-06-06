# Open Cisterna - An Open Source cistern sensor.

[![ASLv2](https://img.shields.io/badge/license-Apache%20License%20v2.0-green.svg)](http://www.apache.org/licenses/LICENSE-2.0.html)

## Building from source

```bash
# download Open Cistena code
$ git clone https://github.com/slotbaer/open-cisterna.git
$ cd open-cisterna

# build in release mode
$ cargo build --release
```

This will produce an executable in the `./target/release` subdirectory.

In case you encounter errors please refer to the [Troubleshooting](https://github.com/slotbaer/open-cisterna/wiki/Troubleshooting) wiki page.

## Adjust Log Level

Logging is controlled using environment variables (for details see the offical
doc.rs [documentation](https://docs.rs/env_logger/0.5.10/env_logger/)). The
default log level is `error`. To set the log level to `info` for example use

```bash
$ RUST_LOG=info ./target/release/open-cisterna
```
