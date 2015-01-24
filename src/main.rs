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
    beanstalkd-cli [options] put <message>
    beanstalkd-cli [options] top
    beanstalkd-cli [options] stats [<key>]
    beanstalkd-cli [(--help | --version)]

Options:
    -h, --host=<host>  Hostname of the beanstalkd server [default: localhost]
    -p, --port=<port>  Port of the beanstalkd server [default: 11300]
    --help             Display this message
    -v, --version      Print version info and exit
";

#[derive(RustcDecodable, Debug)]
struct Args {
    flag_host: String,
    flag_port: u16,
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

    if ! (args.cmd_put || args.cmd_top || args.cmd_stats) {
        println!("{}", USAGE.trim());
        return;
    }

    let host = args.flag_host.as_slice();
    let port = args.flag_port;
    let mut beanstalkd = Beanstalkd::connect(host, port).ok().expect("Server not running");

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
    }
}
