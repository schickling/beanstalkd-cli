# beanstalkd-cli [![Build Status](https://travis-ci.org/schickling/beanstalkd-cli.svg?branch=master)](https://travis-ci.org/schickling/beanstalkd-cli)
Simple to use commandline tool for [Beanstalkd](https://github.com/kr/beanstalkd)

## Download

You can download a prebuilt binary [here](https://github.com/schickling/beanstalkd-cli/releases).

## Usage
```
Beanstalkd CLI

Usage:
    beanstalkd-cli [options] put <message>
    beanstalkd-cli [options] pop
    beanstalkd-cli [options] monitor
    beanstalkd-cli [options] stats [<key>]
    beanstalkd-cli [(--help | --version)]

Commands:
    put <messsage>     Writes a message to the queue
    pop                Removes and prints the next message in the queue
    monitor            Live monitoring of the queue
    stats [<key>]      Prints all stats or stats for a specific key

Options:
    -h, --host=<host>  Hostname of the beanstalkd server [default: localhost]
    -p, --port=<port>  Port of the beanstalkd server [default: 11300]
    --help             Display this message
    -v, --version      Print version info and exit
```

## Development

This CLI tool in written in Rust and based on the [rust-beanstalkd library](https://github.com/schickling/rust-beanstalkd).

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
