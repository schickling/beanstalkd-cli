# beanstalkd-cli [![Build Status](https://travis-ci.org/schickling/beanstalkd-cli.svg?branch=master)](https://travis-ci.org/schickling/beanstalkd-cli)
Simple to use commandline tool for [Beanstalkd](https://github.com/kr/beanstalkd)

## Download

You can download a prebuilt binary [here](https://github.com/schickling/beanstalkd-cli/releases).

## Usage
```
Beanstalkd CLI

Usage:
    beanstalkd-cli [options] put <message>
    beanstalkd-cli [options] top
    beanstalkd-cli [options] stats [<key>]
    beanstalkd-cli [(--help | --version)]

Options:
    -h, --host=<host>  Hostname of the beanstalkd server [default: localhost]
    -p, --port=<port>  Port of the beanstalkd server [default: 11300]
    --help             Display this message
    -v, --version      Print version info and exit
```

## Development

This CLI tool in written in Rust and based on the [rust-beanstalkd library](https://github.com/schickling/rust-beanstalkd).
