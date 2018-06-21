# Open Cisterna - An Open Source cistern sensor.

[![ASLv2](https://img.shields.io/badge/license-Apache%20License%20v2.0-green.svg)](http://www.apache.org/licenses/LICENSE-2.0.html)

## Building

### From Source

```bash
# download Open Cistena code
$ git clone https://github.com/slotbaer/open-cisterna.git
$ cd open-cisterna

# build in release mode
$ cargo build --release
```

This will produce an executable in the `./target/release` subdirectory.

#### Troubleshooting

In case you encounter errors please refer to the [Troubleshooting](https://github.com/slotbaer/open-cisterna/wiki/Troubleshooting) wiki page.

### Debian Package

Sources for building a very basic Debian package are located in the `debian`
folder. To build the package invoke the following commands from the root of
the source tree

```bash
$ cargo build --release
$ sudo apt-get install debhelper
$ dpkg-buildpackage -us -uc -b
```

The package will have been built one level below the root of the source tree.

## Usage

We recommend using the Debian package as it will setup a systemd service automatically.

### Installation

You can install the package by invoking

```bash
$ sudo dpkg -i *.deb
```
### Checking the Service Status

The status of the service can be checked using

```bash
$ sudo systemctl status opencisterna.service
```

### Uninstallation

Uninstallation is accomplished by invoking

```bash
$ sudo dpkg -r opencisterna
```

### Configuration

#### Adjust Log Level

Logging is controlled using environment variables (for details see the offical
doc.rs [documentation](https://docs.rs/env_logger/0.5.10/env_logger/)). The
default log level is `error`. To set the log level to `debug` for example use

```bash
$ RUST_LOG=debug ./target/release/open-cisterna
```

For the systemd service you have to modify the service configuration. This can
be accomplished using

```bash
$ sudo systemctl edit opencisterna.service
```

This will create an overlay file in which you can define new values for the
environment variables for the `open-cisterna` process. To adjust the logging
level to `debug` you should add the following line to the file

```bash
Environment="RUST_LOG=debug"
```
