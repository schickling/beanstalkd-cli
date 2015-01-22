#![allow(unstable)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;
extern crate beanstalkd;

use docopt::Docopt;
use beanstalkd::Beanstalkd;

static VERSION: &'static str = "0.0.0";
static USAGE: &'static str = "
Command line Beanstalkd tool

Usage:
    beanstalkd-cli put <message>
    beanstalkd-cli stats [<key>]
    beanstalkd-cli [options]

Options:
    -h, --help       Display this message
    -v, --version    Print version info and exit
";

#[derive(RustcDecodable, Show)]
struct Args {
    cmd_put: bool,
    cmd_stats: bool,
    arg_key: Option<String>,
    arg_message: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.version(Some(VERSION.to_string())).decode())
                            .unwrap_or_else(|e| e.exit());

    let mut beanstalkd = Beanstalkd::localhost().unwrap();

    if args.cmd_put {
        let message = args.arg_message.as_slice();
        beanstalkd.put(message, 0, 0, 10000);
    } else if args.cmd_stats {
        let stats = beanstalkd.stats().unwrap();
        if args.arg_key.is_some() {
            println!("{}", stats.get(&args.arg_key.unwrap()).unwrap());
        } else {
            for (key, value) in stats.iter() {
                println!("{}: {}", key, value);
            }
        }
    } else {
        println!("{}", USAGE.trim());
    }
}
