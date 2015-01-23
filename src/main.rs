#![allow(unstable)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;
extern crate beanstalkd;

use docopt::Docopt;
use beanstalkd::Beanstalkd;

mod commands;

static VERSION: &'static str = "0.0.0";
static USAGE: &'static str = "
Command line Beanstalkd tool

Usage:
    beanstalkd-cli put <message>
    beanstalkd-cli top
    beanstalkd-cli stats [<key>]
    beanstalkd-cli [options]

Options:
    -h, --help       Display this message
    -v, --version    Print version info and exit
";

#[derive(RustcDecodable, Show)]
struct Args {
    cmd_put: bool,
    arg_message: String,
    cmd_top: bool,
    cmd_stats: bool,
    arg_key: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.version(Some(VERSION.to_string())).decode())
                            .unwrap_or_else(|e| e.exit());

    let mut beanstalkd = Beanstalkd::localhost().unwrap();

    if args.cmd_put {
        commands::put::put(&mut beanstalkd, args.arg_message);
    } else if args.cmd_top {
        commands::top::top(&mut beanstalkd);
    } else if args.cmd_stats {
        if args.arg_key.is_some() {
            commands::stats::get(&mut beanstalkd, args.arg_key.unwrap());
        } else {
            commands::stats::all(&mut beanstalkd);
        }
    } else {
        println!("{}", USAGE.trim());
    }
}
