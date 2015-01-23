# beanstalkd-cli [![Build Status](https://travis-ci.org/schickling/beanstalkd-cli.svg?branch=master)](https://travis-ci.org/schickling/beanstalkd-cli)
Command line Beanstalkd tool

## Download

You can download a prebuilt binary [here](https://github.com/schickling/beanstalkd-cli/releases).

## Usage
```
Command line Beanstalkd tool

Usage:
    beanstalkd-cli put <message>
    beanstalkd-cli top
    beanstalkd-cli stats [<key>]
    beanstalkd-cli [options]

Options:
    -h, --help       Display this message
    -v, --version    Print version info and exit
```

## Development

This CLI tool in written in Rust and based on the [rust-beanstalkd librar](https://github.com/schickling/rust-beanstalkd).
