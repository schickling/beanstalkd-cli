#![feature(core)]
#![feature(std_misc)]
#![feature(old_io)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;
extern crate beanstalkd;

use docopt::Docopt;
use beanstalkd::Beanstalkd;

mod commands;

static VERSION: &'static str = "0.0.0";
static USAGE: &'static str = "
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
";

#[derive(RustcDecodable, Debug)]
struct Args {
    flag_host: String,
    flag_port: u16,
    cmd_put: bool,
    arg_message: String,
    cmd_pop: bool,
    cmd_monitor: bool,
    cmd_stats: bool,
    arg_key: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.version(Some(VERSION.to_string())).decode())
                            .unwrap_or_else(|e| e.exit());

    if ! (args.cmd_put || args.cmd_pop || args.cmd_monitor || args.cmd_stats) {
        println!("{}", USAGE.trim());
        return;
    }

    let host = args.flag_host.as_slice();
    let port = args.flag_port;
    let mut beanstalkd = Beanstalkd::connect(host, port).ok().expect("Server not running");

    if args.cmd_put {
        commands::put::put(&mut beanstalkd, args.arg_message);
    } else if args.cmd_pop {
        commands::pop::pop(&mut beanstalkd);
    } else if args.cmd_monitor {
        commands::monitor::monitor(&mut beanstalkd);
    } else if args.cmd_stats {
        if args.arg_key.is_some() {
            commands::stats::get(&mut beanstalkd, args.arg_key.unwrap());
        } else {
            commands::stats::all(&mut beanstalkd);
        }
    }
}
